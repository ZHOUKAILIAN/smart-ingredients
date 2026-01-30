# loki.smartingredients.my

## Purpose
- Loki API endpoint for logs storage/query.

## DNS
- A record: `loki.smartingredients.my` → `<SERVER_IP>`
- Cloudflare proxy optional; if enabled use **Full (strict)** SSL.

## Nginx
- Upstream: `127.0.0.1:3100`
- Health check (Nginx-level): `https://loki.smartingredients.my/health`
- Loki status: `https://loki.smartingredients.my/loki/api/v1/status/buildinfo`

Example (server block excerpt):
```nginx
server_name loki.smartingredients.my;
proxy_pass http://127.0.0.1:3100;
```

## TLS / Certbot
- Shared certificate with other domains:
  - `smartingredients.my`
  - `api.smartingredients.my`
  - `grafana.smartingredients.my`
  - `loki.smartingredients.my`

## Dependencies
- Monitoring stack (`docs/deployment/monitoring/docker-compose.monitoring.yml`)
- Loki container on port 3100
- Nginx reverse proxy
- Let’s Encrypt certificate for this domain

## Related domains
- Root: `./smartingredients.my.md`
- API: `./api.smartingredients.my.md`
- Grafana: `./grafana.smartingredients.my.md`
- Ops overview: `../server-deploy.md`
