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

## Tips

- Use JPEG/PNG/WebP for uploads.
- Large images may take longer due to OCR + LLM latency.
- If OCR fails, the analysis status stays `failed` and the error is returned.
