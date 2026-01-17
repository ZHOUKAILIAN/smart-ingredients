# Backend Integration Guide

This guide shows how to call the backend APIs end-to-end with curl.

## 1) Upload image

```bash
curl -X POST http://localhost:3000/api/v1/analysis/upload \
  -F "file=@/path/to/ingredients.jpg"
```

Expected response:

```json
{
  "analysis_id": "UUID",
  "image_url": "/uploads/UUID.jpg"
}
```

## 2) Trigger OCR + LLM analysis

```bash
curl -X POST http://localhost:3000/api/v1/analysis/UUID/analyze
```

Expected response:

```json
{
  "analysis_id": "UUID",
  "status": "completed"
}
```

## 3) Query analysis result

```bash
curl http://localhost:3000/api/v1/analysis/UUID
```

Expected response includes:

- `status`
- `image_url`
- `ocr_text`
- `result` (LLM JSON)

## 4) Verify data in Postgres

Option A: via container `psql`:

```bash
docker exec -i smart-ingredients-db psql -U smart_ingredients -d smart_ingredients \
  -c "select id, status, image_url, text, result, created_at, updated_at from analyses order by created_at desc limit 5;"
```

Option B: via local `psql`:

```bash
psql "postgresql://smart_ingredients:smart_ingredients@localhost:5432/smart_ingredients" \
  -c "select id, status, image_url, text, result, created_at, updated_at from analyses order by created_at desc limit 5;"
```

## Tips

- Use JPEG/PNG/WebP for uploads.
- Large images may take longer due to OCR + LLM latency.
- If OCR fails, the analysis status stays `failed` and the error is returned.
