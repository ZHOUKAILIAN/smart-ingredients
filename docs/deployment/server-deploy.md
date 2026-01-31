# Server Deployment Guide (Ubuntu + Docker)

This guide helps you deploy the backend services to a Linux server using Docker.
It does **not** include any secrets. Do **not** paste passwords in chats or commit them to git.

## Prerequisites

- Ubuntu 20.04+ (or compatible Linux)
- Docker + Docker Compose installed
- A domain or server IP with ports open:
  - `80` / `443` (Nginx + HTTPS)
  - `3000` (backend API)
  - `3001` (Grafana, if exposed directly)
  - `3100` (Loki, if exposed directly)
  - `5432` (Postgres, optional if you need remote access)
  - `6379` (Redis, optional if you need remote access)
  - `8000` (OCR service, optional if you need remote access)

## 1) Install Docker (if not installed)

```bash
sudo apt update
sudo apt install -y ca-certificates curl gnupg
sudo install -m 0755 -d /etc/apt/keyrings
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg
echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu \
  $(. /etc/os-release && echo $VERSION_CODENAME) stable" | \
  sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
sudo apt update
sudo apt install -y docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin
sudo usermod -aG docker $USER
newgrp docker
```

## 2) Upload the project to the server

You can use `git` (recommended) or `scp`.

### Option A: Git
```bash
git clone <YOUR_REPO_URL> smart-ingredients
cd smart-ingredients
```

### Option B: SCP
```bash
scp -r /path/to/smart-ingredients user@server:/opt/smart-ingredients
ssh user@server
cd /opt/smart-ingredients
```

## 3) Configure environment variables

Copy `.env.example` and edit values as needed:

```bash
cp .env.example .env
```

Edit `.env` to set:
- `DATABASE_URL`
- `REDIS_URL`
- `OCR_SERVICE_URL`
- any API keys (keep them **private**)

## 4) Build and start services

```bash
docker compose up -d --build postgres redis ocr backend
```

Check container status:

```bash
docker compose ps
```

## 5) Run database migrations

The backend starts with migrations. If you need to run manually:

```bash
docker compose exec backend bash -lc "cd /app && ./migrate.sh"
```

## 6) Verify health

```bash
curl http://<SERVER_IP>:3000/health
```

## 7) Optional: run behind Nginx (HTTP/2 + TLS)

If you want a custom domain, HTTP/2, and TLS, proxy `http://127.0.0.1:3000` via Nginx.
Cloudflare → Origin HTTP/2 requires TLS + ALPN on the origin.

### 7.1 Install Nginx + Certbot

```bash
sudo apt update
sudo apt install -y nginx certbot python3-certbot-nginx
```

### 7.2 Create an Nginx site

Replace `example.com` with your domain.

```bash
sudo tee /etc/nginx/sites-available/smart-ingredients <<'EOF'
upstream smart_ingredients_backend {
  server 127.0.0.1:3000;
  keepalive 64;
}

server {
  listen 80;
  server_name example.com;
  return 301 https://$host$request_uri;
}

server {
  listen 443 ssl http2;
  server_name example.com;

  client_max_body_size 20m;

  ssl_certificate /etc/letsencrypt/live/example.com/fullchain.pem;
  ssl_certificate_key /etc/letsencrypt/live/example.com/privkey.pem;

  location / {
    proxy_pass http://smart_ingredients_backend;
    proxy_http_version 1.1;
    proxy_set_header Connection "";
    proxy_set_header Host $host;
    proxy_set_header X-Real-IP $remote_addr;
    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    proxy_set_header X-Forwarded-Proto $scheme;
  }
}
EOF

Template file in repo: `docs/deployment/nginx/smart-ingredients.conf`
```

Enable the site and reload Nginx:

```bash
sudo ln -s /etc/nginx/sites-available/smart-ingredients /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx
```

### 7.3 Enable HTTPS (Let’s Encrypt)

```bash
sudo certbot --nginx -d example.com
```

### 7.4 Verify

```bash
curl https://example.com/health
```

### 7.5 Cloudflare settings (for end-to-end HTTP/2)

- **SSL/TLS mode**: Full (strict)
- **HTTP/2**: On
- **HTTP/2 to Origin**: On

## 8) Ops notes (domain + HTTPS + monitoring)

This section captures a production-friendly setup that proxies domains to local services
and adds simple health checks at the Nginx layer.

### 8.1 DNS + Cloudflare

Create A records that point to your server IP:

- `smartingredients.my` → `<SERVER_IP>`
- `api.smartingredients.my` → `<SERVER_IP>`
- `grafana.smartingredients.my` → `<SERVER_IP>`
- `loki.smartingredients.my` → `<SERVER_IP>`

If you enable the Cloudflare orange cloud proxy, set **SSL/TLS mode** to **Full (strict)**.

Current production domains (as of 2026-01-30):

- `https://smartingredients.my`
- `https://api.smartingredients.my`
- `https://grafana.smartingredients.my`
- `https://loki.smartingredients.my`

Domain-specific runbooks:

- `docs/deployment/domains/smartingredients.my.md`
- `docs/deployment/domains/api.smartingredients.my.md`
- `docs/deployment/domains/grafana.smartingredients.my.md`
- `docs/deployment/domains/loki.smartingredients.my.md`

### 8.2 Nginx reverse proxy layout (example)

Recommended mapping:

- `smartingredients.my` → `127.0.0.1:3000`
- `api.smartingredients.my` → `127.0.0.1:3000`
- `grafana.smartingredients.my` → `127.0.0.1:3001`
- `loki.smartingredients.my` → `127.0.0.1:3100`

Each site can expose a lightweight health endpoint at `/health`:

```nginx
location = /health {
  default_type text/plain;
  return 200 "ok";
}
```

Useful commands:

```bash
sudo nginx -t
sudo systemctl reload nginx
```

### 8.3 Health checks

- Backend (Nginx): `https://api.<domain>/health`
- Backend (direct): `http://127.0.0.1:3000/health`
- Grafana: `https://grafana.<domain>/health`
- Loki: `https://loki.<domain>/health`
- Loki status: `https://loki.<domain>/loki/api/v1/status/buildinfo`

### 8.4 Monitoring stack (Grafana + Loki + Promtail)

Start the monitoring stack:

```bash
docker compose -f docs/deployment/monitoring/docker-compose.monitoring.yml up -d
```

Defaults:

- Grafana: `http://<SERVER_IP>:3001` (or `https://grafana.<domain>`)
- Loki: `http://<SERVER_IP>:3100` (or `https://loki.<domain>`)

### 8.5 Certbot renewal

```bash
sudo certbot renew --dry-run
```

### 8.6 Frontend API domain (build time)

For Android builds, set API base at build time:

```bash
API_BASE="https://api.<domain>" cargo tauri android build --apk true
```

## Troubleshooting

- View logs:
  ```bash
  docker compose logs -f backend
  docker compose logs -f ocr
  ```
- Rebuild after changes:
  ```bash
  docker compose up -d --build backend ocr
  ```
