# Server Deployment Guide (Ubuntu + Docker)

This guide helps you deploy the backend services to a Linux server using Docker.
It does **not** include any secrets. Do **not** paste passwords in chats or commit them to git.

## Prerequisites

- Ubuntu 20.04+ (or compatible Linux)
- Docker + Docker Compose installed
- A domain or server IP with ports open:
  - `3000` (backend API)
  - `5432` (Postgres, optional if you need remote access)
  - `6379` (Redis, optional if you need remote access)
  - `8001` (OCR service)

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

## 7) Optional: run behind Nginx

If you want a custom domain and TLS, proxy `http://127.0.0.1:3000` via Nginx.

### 7.1 Install Nginx + Certbot

```bash
sudo apt update
sudo apt install -y nginx certbot python3-certbot-nginx
```

### 7.2 Create an Nginx site

Replace `example.com` with your domain.

```bash
sudo tee /etc/nginx/sites-available/smart-ingredients <<'EOF'
server {
  listen 80;
  server_name example.com;

  client_max_body_size 20m;

  location / {
    proxy_pass http://127.0.0.1:3000;
    proxy_http_version 1.1;
    proxy_set_header Host $host;
    proxy_set_header X-Real-IP $remote_addr;
    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    proxy_set_header X-Forwarded-Proto $scheme;
  }
}
EOF
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

