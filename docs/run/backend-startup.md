# Backend Startup Guide

## Prerequisites

- Docker and Docker Compose
- `.env` file present at repo root (copy from `.env.example` and fill in values)

## Start services (Docker Compose)

1. Start PostgreSQL and backend:

```bash
docker compose up -d postgres backend
```

2. Check logs:

```bash
docker compose logs --tail=200 backend
```

3. Verify health:

```bash
curl http://localhost:3000/health
```

## Stop services

```bash
docker compose down
```

## Common env vars

- `DATABASE_URL`: Postgres connection string (Docker uses the compose value)
- `LLM_PROVIDER`: `deepseek` (current supported)
- `DEEPSEEK_API_KEY`: LLM key
- `UPLOAD_DIR`: Local uploads directory (default `uploads`)
- `MAX_UPLOAD_BYTES`: Max upload size (default `10485760`)

## Notes

- OCR runs inside the backend container via Tesseract.
- The backend binds to `0.0.0.0:3000` in Docker; use `http://localhost:3000`.
- If you change `.env`, restart the backend container.
