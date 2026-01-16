# Technical Design - Smart Ingredients

## System Architecture

### High-Level Architecture

```text
┌─────────────────────────────────────────────────────────────────┐
│                         Frontend                             │
│                     (Tauri + Leptos)                         │
└────────────────────────────┬────────────────────────────────────┘
                             │ HTTP/REST
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│                         Backend                              │
│                        (Axum + Tokio)                        │
└───────┬─────────────┬─────────────┬─────────────┬──────────┘
        │             │             │             │
        ▼             ▼             ▼             ▼
   ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐
   │  OCR    │  │   LLM   │  │   DB    │  │  Cache  │
   │ Service │  │ Service │  │Postgres │  │  Redis  │
   └─────────┘  └─────────┘  └─────────┘  └─────────┘
```

### Component Overview

| Component | Technology | Responsibility |
|-----------|------------|----------------|
| Frontend | Tauri + Leptos | UI, camera, image handling |
| Backend API | Axum | Request handling, orchestration |
| OCR Service | PaddleOCR/Tesseract | Text extraction from images |
| LLM Service | DeepSeek/智谱 AI | Ingredient health analysis |
| Database | PostgreSQL | Persistent data storage |
| Cache | Redis | Session, rate limiting, result caching |

## API Design

### Base URL

```
Development: http://localhost:3000/api
Production: https://api.smart-ingredients.com/api
```

### Endpoints

#### 1. Upload Image

```
POST /api/v1/analysis/upload
Content-Type: multipart/form-data

Request:
{
  "image": <binary file>,
  "filename": "product.jpg"
}

Response (201):
{
  "id": "uuid",
  "status": "pending",
  "image_url": "https://..."
}
```

#### 2. Get Analysis Status

```
GET /api/v1/analysis/{id}

Response (200):
{
  "id": "uuid",
  "status": "completed",
  "ocr_text": "...",
  "result": {
    "health_score": 75,
    "ingredients": [...],
    "warnings": [...]
  }
}
```

#### 3. List History

```
GET /api/v1/analysis/history?page=1&limit=20

Response (200):
{
  "total": 100,
  "page": 1,
  "limit": 20,
  "items": [...]
}
```

#### 4. Get Analysis Details

```
GET /api/v1/analysis/{id}/details

Response (200):
{
  "id": "uuid",
  "image_url": "...",
  "ocr_text": "...",
  "result": {...},
  "created_at": "2024-01-01T00:00:00Z",
  "is_favorite": false
}
```

## Database Schema

### Tables

#### analyses

```sql
CREATE TABLE analyses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    image_url VARCHAR(512) NOT NULL,
    ocr_text TEXT,
    status VARCHAR(50) NOT NULL,
    health_score INTEGER,
    result JSONB,
    error_message TEXT,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX idx_analyses_status ON analyses(status);
CREATE INDEX idx_analyses_created_at ON analyses(created_at DESC);
```

#### ingredients

```sql
CREATE TABLE ingredients (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) UNIQUE NOT NULL,
    category VARCHAR(100),
    health_risk VARCHAR(50),
    description TEXT,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX idx_ingredients_name ON ingredients(name);
CREATE INDEX idx_ingredients_category ON ingredients(category);
```

#### favorites

```sql
CREATE TABLE favorites (
    id SERIAL PRIMARY KEY,
    analysis_id UUID REFERENCES analyses(id) ON DELETE CASCADE,
    user_id VARCHAR(255), -- For future user system
    created_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(analysis_id, user_id)
);
```

## Data Flow

### Image Analysis Flow

```text
1. User captures/selects image
   ↓
2. Frontend uploads to /api/v1/analysis/upload
   ↓
3. Backend saves to object storage
   ↓
4. Backend queues OCR task (Redis)
   ↓
5. Worker processes OCR
   ↓
6. Backend sends text to LLM
   ↓
7. LLM returns analysis
   ↓
8. Backend saves results to database
   ↓
9. Frontend polls for completion
   ↓
10. Display results to user
```

## Error Handling

### Error Codes

| Code | Description |
|------|-------------|
| 400 | Bad Request (invalid input) |
| 401 | Unauthorized (future) |
| 413 | Payload Too Large (>10MB) |
| 415 | Unsupported Media Type |
| 429 | Rate Limit Exceeded |
| 500 | Internal Server Error |

### Error Response Format

```json
{
  "error": {
    "code": "INVALID_IMAGE_FORMAT",
    "message": "Only JPEG, PNG, and WebP are supported",
    "details": {}
  }
}
```

## Security Design

### Authentication (Future)

- JWT-based authentication
- Token expiration: 7 days
- Refresh token rotation

### Authorization

- Public endpoints: upload, status check
- Protected endpoints: history, favorites (future)

### Data Protection

- Images encrypted at rest
- Database encryption at rest
- TLS for all API communication
- API keys stored in environment variables

### Rate Limiting

- Upload: 10 requests/minute per IP
- Status check: 100 requests/minute per IP

## Caching Strategy

### Redis Cache Keys

```
analysis:{id}          # Analysis result (TTL: 1 hour)
ocr:{image_hash}       # OCR result (TTL: 24 hours)
rate_limit:{ip}        # Rate limit counter (TTL: 60 seconds)
```

### Cache Invalidation

- Invalidate on analysis completion
- Invalidate on manual refresh
- Time-based expiration

## Deployment Architecture

### Development Environment

```text
┌─────────────────────────────────────┐
│         Docker Compose            │
│                                 │
│  ┌──────────┐  ┌──────────┐   │
│  │ Frontend │  │ Backend  │   │
│  │ (dev)    │  │ (dev)    │   │
│  └──────────┘  └──────────┘   │
│       │             │          │
│       └─────────────┼──────────┘
│                     ▼          │
│              ┌──────────┐       │
│              │PostgreSQL│       │
│              │  Redis   │       │
│              └──────────┘       │
└─────────────────────────────────────┘
```

### Production Environment

```text
┌─────────────────────────────────────────────────────┐
│                  Load Balancer                   │
└────────────┬────────────────────┬───────────────┘
             │                    │
             ▼                    ▼
      ┌──────────┐          ┌──────────┐
      │ Backend  │          │ Backend  │
      │ Pod #1   │          │ Pod #2   │
      └──────────┘          └──────────┘
             │                    │
             └────────┬───────────┘
                      ▼
            ┌──────────────────┐
            │   PostgreSQL    │
            │   (Primary)    │
            └──────────────────┘
                      │
             ┌──────────┴──────────┐
             ▼                     ▼
      ┌──────────┐          ┌──────────┐
      │ Replica  │          │  Redis   │
      │ (Read)   │          │ Cluster  │
      └──────────┘          └──────────┘
```

## Monitoring & Observability

### Metrics

- Request count and duration
- OCR success rate
- LLM response time
- Error rates by endpoint

### Logging

- Structured JSON logs
- Log levels: ERROR, WARN, INFO, DEBUG
- Centralized log aggregation

### Health Checks

```
GET /health
Response: { "status": "ok", "timestamp": "..." }
```

## Technology Decisions

### Why Rust?

- Performance: Zero-cost abstractions, no GC
- Safety: Memory safety at compile time
- Concurrency: Tokio async runtime
- Type sharing: Frontend and backend share types

### Why Axum?

- Type-safe routing
- Extractor-based request handling
- Tower middleware ecosystem
- Async-first design

### Why PostgreSQL?

- ACID compliance
- JSONB support for flexible data
- Mature tooling
- Scalable to production needs

### Why Redis?

- In-memory performance
- Rich data structures
- Pub/sub for real-time features
- Distributed caching
