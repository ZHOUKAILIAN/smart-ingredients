# Email Login Demo (Code-based)

This is a standalone demo that shows an email verification code login flow.
It does not touch your existing app.

## Run

```bash
python3 server.py
```

Then open:

```
http://127.0.0.1:8081/
```

## Demo mode (no SMTP)

If you do not set SMTP env vars, the server will print the code to console and
return a `debug_code` in the response.

## SMTP mode (real email)

Set these environment variables before running (or create `config.json`, see below):

```bash
export SMTP_HOST="smtp.example.com"
export SMTP_PORT="587"
export SMTP_USER="no-reply@example.com"
export SMTP_PASS="your-smtp-password"
export SMTP_FROM="no-reply@example.com"
```

Then run `python3 server.py` and the code will be sent by email.

## Optional config.json

Create `config.json` in this folder:

```json
{
  "smtp_host": "smtp.example.com",
  "smtp_port": 587,
  "smtp_user": "no-reply@example.com",
  "smtp_pass": "your-smtp-password",
  "smtp_from": "no-reply@example.com",
  "smtp_ssl": false,
  "smtp_starttls": true
}
```

Notes:
- Use `smtp_ssl: true` for port 465.
- Use `smtp_starttls: true` for port 587.
