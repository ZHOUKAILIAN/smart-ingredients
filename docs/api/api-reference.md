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

## Confirm OCR Text (Start LLM)

`POST /api/v1/analysis/{id}/confirm`

Confirm OCR text and start LLM analysis.

### Path Params

- `id`: analysis UUID

### Request

```json
{
  "confirmed_text": "识别文本...",
  "preference": "normal"
}
```

### Response

```json
{
  "id": "uuid",
  "status": "completed",
  "ocr_text": "识别文本...",
  "result": {
    "health_score": 85,
    "summary": "配料以茶叶提取物为主，含少量甜味剂，整体风险较低。",
    "table": [
      {
        "name": "乌龙茶",
        "category": "nutrition",
        "function": "提供茶香与基础风味",
        "risk_level": "low",
        "note": ""
      }
    ],
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

### Response Notes

- When `result.summary` is empty, the backend may provide a short default summary (e.g. based on ingredient count).
- When `result.table` is empty, clients can fall back to `result.ingredients` to render a basic table.
- While analysis is running, `status` will be `pending` or `processing` and `result` may be `null`.
- LLM analysis is triggered by `POST /api/v1/analysis/{id}/confirm`.

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

## Community

### Create Community Post

`POST /api/v1/community/posts`

Create a community post (structure + optional card image).

#### Request

- Content-Type: `multipart/form-data`
- Field: `payload` (JSON string)
- Field: `card_image` (optional image file)

Payload example:

```json
{
  "author_type": "anonymous",
  "share_token": "client-generated-token",
  "source_analysis_id": "uuid",
  "summary_text": "配料以茶叶提取物为主，整体风险较低。",
  "health_score": 85,
  "ingredients_raw": "水、乌龙茶、白砂糖...",
  "card_payload": {
    "health_score": 85,
    "summary": "配料以茶叶提取物为主，整体风险较低。",
    "recommendation": "建议适量饮用",
    "ingredients": [
      { "name": "乌龙茶", "risk_level": "low", "description": "天然茶叶提取物", "is_focus": false }
    ],
    "warnings": [],
    "preference_label": "普通人群"
  }
}
```

#### Response

```json
{
  "id": "uuid",
  "created_at": "2026-02-19T12:00:00Z",
  "card_image_url": "/uploads/community/xxx.png"
}
```

### List Community Posts

`GET /api/v1/community/posts`

#### Query Params

- `page`: optional, default 1
- `limit`: optional, default 20, range 1-100

#### Response

```json
{
  "total": 120,
  "page": 1,
  "limit": 20,
  "items": [
    {
      "id": "uuid",
      "summary_text": "配料以茶叶提取物为主，整体风险较低。",
      "health_score": 85,
      "card_image_url": "/uploads/community/xxx.png",
      "author_label": "匿名用户",
      "created_at": "2026-02-19T12:00:00Z"
    }
  ]
}
```

### Get Community Post Detail

`GET /api/v1/community/posts/{id}`

#### Response

```json
{
  "id": "uuid",
  "summary_text": "配料以茶叶提取物为主，整体风险较低。",
  "health_score": 85,
  "ingredients_raw": "水、乌龙茶、白砂糖...",
  "card_payload": {
    "health_score": 85,
    "summary": "配料以茶叶提取物为主，整体风险较低。",
    "recommendation": "建议适量饮用",
    "ingredients": [],
    "warnings": [],
    "preference_label": null
  },
  "card_image_url": null,
  "author_label": "匿名用户",
  "created_at": "2026-02-19T12:00:00Z"
}
```

### Delete Community Post

`DELETE /api/v1/community/posts/{id}`

#### Request Body

```json
{ "share_token": "client-generated-token" }
```

#### Response

```json
{ "deleted": true }
```

## Common Types

### `AnalysisStatus`

`pending | processing | completed | failed`

### `TableRow`

```json
{
  "name": "string",
  "category": "string",
  "function": "string",
  "risk_level": "low | medium | high | unknown",
  "note": "string"
}
```
