# api.smartingredients.my

## Purpose
- Backend API entrypoint for web/mobile clients.

## DNS
- A record: `api.smartingredients.my` → `<SERVER_IP>`
- Cloudflare proxy optional; if enabled use **Full (strict)** SSL.

## Nginx
- Upstream: `127.0.0.1:3000`
- Health check (Nginx-level): `https://api.smartingredients.my/health`
- Backend health (direct): `http://127.0.0.1:3000/health`

Example (server block excerpt):
```nginx
server_name api.smartingredients.my;
proxy_pass http://127.0.0.1:3000;
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
