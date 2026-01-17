# API Reference

Base URL: `http://localhost:3000`

## Upload Image

`POST /api/v1/analysis/upload`

Upload an image for OCR + LLM analysis.

### Request

- Content-Type: `multipart/form-data`
- Field: `file` (image file)
- Supported types: `image/jpeg`, `image/png`, `image/webp`
- Max size: 10MB

### Response

```json
{
  "id": "uuid",
  "status": "pending",
  "image_url": "/uploads/xxx.jpg"
}
```

## Analyze Image

`POST /api/v1/analysis/{id}/analyze`

Run OCR and LLM analysis for an uploaded image.

### Path Params

- `id`: analysis UUID

### Response

```json
{
  "id": "uuid",
  "status": "completed",
  "ocr_text": "识别文本...",
  "result": {
    "health_score": 85,
    "ingredients": [
      {
        "name": "乌龙茶",
        "category": "nutrition",
        "risk_level": "low",
        "description": "天然茶叶提取物"
      }
    ],
    "warnings": [
      {
        "warning_type": "过敏原提示",
        "ingredients": ["乌龙茶"],
        "message": "对咖啡因敏感的人群需注意适量饮用"
      }
    ],
    "recommendation": "建议文本..."
  },
  "error_message": null,
  "created_at": "2026-01-17T05:40:56.802230+00:00"
}
```

## Get Analysis

`GET /api/v1/analysis/{id}`

Fetch analysis result by id.

### Path Params

- `id`: analysis UUID

### Response

Same shape as the analyze response.

## History

`GET /api/v1/analysis/history`

List history items (paginated).

### Query Params

- `page`: optional, default 1
- `limit`: optional, default 20, range 1-100

### Response

```json
{
  "total": 1,
  "page": 1,
  "limit": 20,
  "items": [
    {
      "id": "uuid",
      "image_url": "/uploads/xxx.jpg",
      "health_score": 85,
      "created_at": "2026-01-17T05:40:56.802230+00:00",
      "is_favorite": false
    }
  ]
}
```

## Common Types

### `AnalysisStatus`

`pending | processing | completed | failed`
