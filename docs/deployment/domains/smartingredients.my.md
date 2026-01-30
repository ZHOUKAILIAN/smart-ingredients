# smartingredients.my

## Purpose
- Primary domain; currently proxies to backend API (same as `api.smartingredients.my`).

## DNS
- A record: `smartingredients.my` → `<SERVER_IP>`
- Cloudflare proxy (orange cloud) optional; if enabled use **Full (strict)** SSL.

## Nginx
- Upstream: `127.0.0.1:3000`
- Health check (Nginx-level): `https://smartingredients.my/health`

Example (server block excerpt):
```nginx
server_name smartingredients.my;
proxy_pass http://127.0.0.1:3000;
```

## TLS / Certbot
- Certificate is shared with other domains:
  - `smartingredients.my`
  - `api.smartingredients.my`
  - `grafana.smartingredients.my`
  - `loki.smartingredients.my`

## Dependencies
- Backend container on port 3000
- Nginx reverse proxy
- Let’s Encrypt certificate for this domain

## Related domains
- API: `./api.smartingredients.my.md`
- Grafana: `./grafana.smartingredients.my.md`
- Loki: `./loki.smartingredients.my.md`
- Ops overview: `../server-deploy.md`
