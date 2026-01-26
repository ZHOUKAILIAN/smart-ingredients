#!/usr/bin/env python3
import json
from http.server import BaseHTTPRequestHandler, HTTPServer


class Handler(BaseHTTPRequestHandler):
    def do_POST(self):
        if self.path != "/v1/chat/completions":
            self.send_response(404)
            self.end_headers()
            return

        content_length = int(self.headers.get("Content-Length", 0))
        _ = self.rfile.read(content_length)

        result = {
            "health_score": 80,
            "summary": "配料整体较为常见，风险较低。",
            "table": [
                {
                    "name": "示例配料",
                    "category": "nutrition",
                    "function": "提供基础营养",
                    "risk_level": "low",
                    "note": ""
                }
            ],
            "ingredients": [
                {
                    "name": "示例配料",
                    "category": "nutrition",
                    "risk_level": "low",
                    "description": "基础营养成分"
                }
            ],
            "warnings": [],
            "overall_assessment": "总体风险较低。",
            "recommendation": "建议适量摄入。",
            "focus_summary": None,
            "focus_ingredients": [],
            "score_breakdown": [
                {
                    "dimension": "nutrition_value",
                    "score": 80,
                    "reason": "营养结构较为平衡"
                }
            ]
        }

        response = {
            "choices": [
                {
                    "message": {
                        "role": "assistant",
                        "content": json.dumps(result, ensure_ascii=False)
                    }
                }
            ]
        }

        body = json.dumps(response, ensure_ascii=False).encode("utf-8")
        self.send_response(200)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)


def main():
    server = HTTPServer(("0.0.0.0", 9009), Handler)
    print("Mock LLM listening on http://0.0.0.0:9009")
    server.serve_forever()


if __name__ == "__main__":
    main()
