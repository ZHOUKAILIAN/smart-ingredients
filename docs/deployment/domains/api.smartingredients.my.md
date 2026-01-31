# api.smartingredients.my

## Purpose
- Backend API entrypoint for web/mobile clients.

## DNS
- A record: `api.smartingredients.my` → `<SERVER_IP>`
- Cloudflare proxy optional; if enabled use **Full (strict)** SSL.

## Nginx (HTTP/2 + TLS)
- Upstream: `127.0.0.1:3000`
- Health check (Nginx-level): `https://api.smartingredients.my/health`
- Backend health (direct): `http://127.0.0.1:3000/health`

Example (server block excerpt):
```nginx
server {
  listen 443 ssl http2;
  server_name api.smartingredients.my;

  ssl_certificate /etc/letsencrypt/live/api.smartingredients.my/fullchain.pem;
  ssl_certificate_key /etc/letsencrypt/live/api.smartingredients.my/privkey.pem;

  location / {
    proxy_pass http://127.0.0.1:3000;
    proxy_http_version 1.1;
    proxy_set_header Connection "";
    proxy_set_header Host $host;
    proxy_set_header X-Real-IP $remote_addr;
    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    proxy_set_header X-Forwarded-Proto $scheme;
  }
}
```

## TLS / Certbot
- Shared certificate with other domains:
  - `smartingredients.my`
  - `api.smartingredients.my`
  - `grafana.smartingredients.my`
  - `loki.smartingredients.my`

## Dependencies
- Backend container on port 3000
- Nginx reverse proxy
- Let’s Encrypt certificate for this domain

## Related domains
- Root: `./smartingredients.my.md`
- Grafana: `./grafana.smartingredients.my.md`
- Loki: `./loki.smartingredients.my.md`
- Ops overview: `../server-deploy.md`
