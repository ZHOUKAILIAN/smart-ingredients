#!/usr/bin/env python3
import json
import os
import random
import re
import time
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from urllib.parse import urlparse
import smtplib
from email.message import EmailMessage

HOST = "127.0.0.1"
PORT = 8081
BASE_DIR = Path(__file__).resolve().parent

EMAIL_RE = re.compile(r"^[^@\s]+@[^@\s]+\.[^@\s]+$")

CODE_TTL_SECONDS = 300
COOLDOWN_SECONDS = 60

codes = {}
cooldowns = {}


def parse_bool(value, default=False) -> bool:
    if value is None:
        return default
    if isinstance(value, bool):
        return value
    return str(value).strip().lower() in {"1", "true", "yes", "y", "on"}


def load_config() -> dict:
    cfg = {}
    path = BASE_DIR / "config.json"
    if path.exists():
        try:
            cfg = json.loads(path.read_text(encoding="utf-8"))
        except json.JSONDecodeError:
            print("[config] invalid JSON in config.json")
    return cfg


CONFIG = load_config()


def get_cfg(key: str, default=None):
    env_key = key.upper()
    if env_key in os.environ:
        return os.environ.get(env_key)
    return CONFIG.get(key, default)


def now() -> int:
    return int(time.time())


def json_response(handler: BaseHTTPRequestHandler, status: int, data: dict) -> None:
    body = json.dumps(data).encode("utf-8")
    handler.send_response(status)
    handler.send_header("Content-Type", "application/json")
    handler.send_header("Content-Length", str(len(body)))
    handler.end_headers()
    handler.wfile.write(body)


def read_json(handler: BaseHTTPRequestHandler) -> dict:
    length = int(handler.headers.get("Content-Length", "0"))
    if length <= 0:
        return {}
    raw = handler.rfile.read(length)
    return json.loads(raw.decode("utf-8"))


def send_email_code(to_email: str, code: str):
    smtp_host = get_cfg("smtp_host")
    smtp_port = int(get_cfg("smtp_port", "587"))
    smtp_user = get_cfg("smtp_user")
    smtp_pass = get_cfg("smtp_pass")
    smtp_from = get_cfg("smtp_from", smtp_user or "")
    smtp_ssl = parse_bool(get_cfg("smtp_ssl"), default=smtp_port == 465)
    smtp_starttls = parse_bool(get_cfg("smtp_starttls"), default=not smtp_ssl)

    if not smtp_host or not smtp_user or not smtp_pass or not smtp_from:
        return False, "missing smtp config"

    msg = EmailMessage()
    msg["Subject"] = "Your login code"
    msg["From"] = smtp_from
    msg["To"] = to_email
    msg.set_content(f"Your verification code is: {code}\\nThis code expires in 5 minutes.")

    try:
        if smtp_ssl:
            with smtplib.SMTP_SSL(smtp_host, smtp_port, timeout=10) as server:
                server.login(smtp_user, smtp_pass)
                server.send_message(msg)
        else:
            with smtplib.SMTP(smtp_host, smtp_port, timeout=10) as server:
                if smtp_starttls:
                    server.starttls()
                server.login(smtp_user, smtp_pass)
                server.send_message(msg)
        return True, None
    except Exception as exc:  # noqa: BLE001
        return False, str(exc)


class Handler(BaseHTTPRequestHandler):
    def do_GET(self) -> None:
        parsed = urlparse(self.path)
        if parsed.path == "/":
            index_path = BASE_DIR / "index.html"
            data = index_path.read_bytes()
            self.send_response(200)
            self.send_header("Content-Type", "text/html; charset=utf-8")
            self.send_header("Content-Length", str(len(data)))
            self.end_headers()
            self.wfile.write(data)
            return

        self.send_response(404)
        self.end_headers()

    def do_POST(self) -> None:
        parsed = urlparse(self.path)
        if parsed.path == "/send":
            self.handle_send()
            return
        if parsed.path == "/verify":
            self.handle_verify()
            return
        self.send_response(404)
        self.end_headers()

    def handle_send(self) -> None:
        try:
            payload = read_json(self)
        except json.JSONDecodeError:
            json_response(self, 400, {"success": False, "message": "invalid json"})
            return

        email = str(payload.get("email", "")).strip()
        if not email or not EMAIL_RE.match(email):
            json_response(self, 400, {"success": False, "message": "invalid email"})
            return

        last_sent = cooldowns.get(email, 0)
        if now() - last_sent < COOLDOWN_SECONDS:
            json_response(self, 429, {"success": False, "message": "cooldown"})
            return

        code = f"{random.randint(0, 999999):06d}"
        expires_at = now() + CODE_TTL_SECONDS
        codes[email] = {"code": code, "expires_at": expires_at}
        cooldowns[email] = now()

        sent, error = send_email_code(email, code)
        if not sent:
            # Demo mode: print code to console and return debug_code
            print(f"[demo] email={email} code={code} error={error}")
            json_response(
                self,
                200,
                {
                    "success": True,
                    "cooldown_seconds": COOLDOWN_SECONDS,
                    "debug_code": code,
                    "delivery": "console",
                    "error": error,
                },
            )
            return

        json_response(
            self,
            200,
            {
                "success": True,
                "cooldown_seconds": COOLDOWN_SECONDS,
                "delivery": "smtp",
            },
        )

    def handle_verify(self) -> None:
        try:
            payload = read_json(self)
        except json.JSONDecodeError:
            json_response(self, 400, {"success": False, "message": "invalid json"})
            return

        email = str(payload.get("email", "")).strip()
        code = str(payload.get("code", "")).strip()
        if not email or not EMAIL_RE.match(email):
            json_response(self, 400, {"success": False, "message": "invalid email"})
            return
        if not code:
            json_response(self, 400, {"success": False, "message": "missing code"})
            return

        record = codes.get(email)
        if not record:
            json_response(self, 400, {"success": False, "message": "code not found"})
            return
        if now() > record["expires_at"]:
            codes.pop(email, None)
            json_response(self, 400, {"success": False, "message": "code expired"})
            return
        if code != record["code"]:
            json_response(self, 400, {"success": False, "message": "code invalid"})
            return

        codes.pop(email, None)
        json_response(self, 200, {"success": True, "message": "login ok"})


def main() -> None:
    server = ThreadingHTTPServer((HOST, PORT), Handler)
    print(f"Email login demo running at http://{HOST}:{PORT}")
    server.serve_forever()


if __name__ == "__main__":
    main()
