# grafana.smartingredients.my

## Purpose
- Grafana UI for observability dashboards.

## DNS
- A record: `grafana.smartingredients.my` → `<SERVER_IP>`
- Cloudflare proxy optional; if enabled use **Full (strict)** SSL.

## Nginx
- Upstream: `127.0.0.1:3001`
- Health check (Nginx-level): `https://grafana.smartingredients.my/health`

Example (server block excerpt):
```nginx
server_name grafana.smartingredients.my;
proxy_pass http://127.0.0.1:3001;
```

## TLS / Certbot
- Shared certificate with other domains:
  - `smartingredients.my`
  - `api.smartingredients.my`
  - `grafana.smartingredients.my`
  - `loki.smartingredients.my`

## Dependencies
- Monitoring stack (`docs/deployment/monitoring/docker-compose.monitoring.yml`)
- Grafana container on port 3001
- Nginx reverse proxy
- Let’s Encrypt certificate for this domain

## Related domains
- Root: `./smartingredients.my.md`
- API: `./api.smartingredients.my.md`
- Loki: `./loki.smartingredients.my.md`
- Ops overview: `../server-deploy.md`
