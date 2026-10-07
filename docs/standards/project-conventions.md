# Project Conventions - Smart Ingredients

## Workspace Structure

### Root Workspace

The project uses a Cargo workspace with three members:

```toml
[workspace]
members = ["frontend", "backend", "shared"]
resolver = "2"
```

### Member Crates

| Crate      | Purpose                    | Dependencies                |
| ---------- | -------------------------- | --------------------------- |
| `frontend` | Tauri desktop application  | `tauri`, `leptos`, `shared` |
| `backend`  | Axum web server            | `axum`, `sqlx`, `shared`    |
| `shared`   | Common types and utilities | `serde`, `serde_json`       |

## Naming Conventions

### File Names

| Type           | Convention                     | Example             |
| -------------- | ------------------------------ | ------------------- |
| Rust files     | `snake_case.rs`                | `image_uploader.rs` |
| Markdown files | `kebab-case.md`                | `api-design.md`     |
| Test files     | `<module>_test.rs` or `tests/` | `service_test.rs`   |

### Database Names

| Type         | Convention               | Example                      |
| ------------ | ------------------------ | ---------------------------- |
| Tables       | `snake_case`             | `analyses`, `ingredients`    |
| Columns      | `snake_case`             | `health_score`, `created_at` |
| Indexes      | `idx_<table>_<columns>`  | `idx_analyses_status`        |
| Foreign keys | `fk_<table>_<reference>` | `fk_favorites_analysis`      |

### API Endpoints

| Type         | Convention   | Example                      |
| ------------ | ------------ | ---------------------------- |
| Paths        | `kebab-case` | `/api/v1/analysis/upload`    |
| Query params | `snake_case` | `?page=1&limit=20`           |
| JSON keys    | `snake_case` | `health_score`, `created_at` |

## Environment Configuration

### Required Environment Variables

#### Backend

```bash
# Database
DATABASE_URL=postgresql://user:pass@localhost:5432/smart_ingredients

# Redis
REDIS_URL=redis://localhost:6379

# Server
SERVER_HOST=0.0.0.0
SERVER_PORT=3000

# OCR Service
# OCR is now fixed to use PaddleOCR service
OCR_PADDLE_URL=http://localhost:8000/ocr
OCR_TIMEOUT=30
OCR_LANG=chi_sim+eng

# LLM Service
LLM_PROVIDER=deepseek
DEEPSEEK_API_KEY=your_api_key
DEEPSEEK_API_URL=https://api.deepseek.com/v1/chat/completions
DEEPSEEK_MODEL=deepseek-chat

# Storage
STORAGE_TYPE=local  # local | oss | s3
LOCAL_STORAGE_PATH=./uploads
OSS_ACCESS_KEY=...
OSS_SECRET_KEY=...
OSS_BUCKET=...
OSS_ENDPOINT=...

# JWT (future)
JWT_SECRET=...
JWT_EXPIRATION=7d
```

#### Frontend

```bash
# API (build-time)
# Required. Set per environment:
# - Debug: http://localhost:3000
# - Release: https://api.smartingredients.my
# Source order: $API_BASE env var > repo root `.env` > `frontend/.env`
API_BASE=http://localhost:3000

# OCR
VITE_OCR_TIMEOUT=30000

# App
VITE_APP_NAME=Smart Ingredients
VITE_MAX_FILE_SIZE=10485760  # 10MB in bytes
```

### Configuration Files

- `.env` - Local development (gitignored)
- `.env.example` - Template (committed)
- `docker-compose.yml` - Development environment

## Branch Strategy

### Main Branches

| Branch    | Purpose                         |
| --------- | ------------------------------- |
| `main`    | Production-ready code           |
| `develop` | Integration branch for features |

### Feature Branches

```
feat/<feature-name>    # New features
fix/<bug-name>        # Bug fixes
hotfix/<issue-name>   # Critical production fixes
```

### Branch Naming Examples

```
feat/image-upload
feat/ocr-integration
fix/timeout-handling
hotfix/security-patch
```

## Code Review Process

### Before Submitting PR

1. Run `cargo fmt`
2. Run `cargo clippy`
3. Run `cargo test`
4. Update documentation if needed
5. Write clear commit messages

### PR Checklist

- [ ] Code follows project standards
- [ ] Tests added/updated
- [ ] Documentation updated
- [ ] No clippy warnings
- [ ] All tests pass
- [ ] Self-reviewed

### PR Template

```markdown
## Description

Brief description of changes

## Type

- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation

## Testing

Describe testing performed

## Checklist

- [ ] Tests pass
- [ ] Documentation updated
- [ ] No clippy warnings
```

## Release Process

### Versioning

Follow Semantic Versioning: `MAJOR.MINOR.PATCH`

- `MAJOR`: Incompatible API changes
- `MINOR`: Backwards-compatible features
- `PATCH`: Backwards-compatible bug fixes

### Release Steps

1. Update version numbers in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Tag release: `git tag v1.0.0`
4. Push tags: `git push --tags`
5. Build and publish artifacts

## Logging Conventions

### Log Format

Use structured logging with consistent fields:

```rust
info!(
    image_id = %id,
    file_size = bytes.len(),
    "Image uploaded successfully"
);
```

### Log Levels

| Level   | Usage                      | Example                 |
| ------- | -------------------------- | ----------------------- |
| `ERROR` | Errors requiring attention | Failed to process image |
| `WARN`  | Unexpected but recoverable | OCR timeout, retrying   |
| `INFO`  | Important events           | User uploaded image     |
| `DEBUG` | Detailed info              | OCR result details      |

### Request Tracing

All HTTP requests should include trace ID:

```rust
use axum::extract::Request;
use uuid::Uuid;

async fn log_request(req: Request, next: Next) -> Response {
    let trace_id = Uuid::new_v4();
    // ... log with trace_id
    next.run(req).await
}
```

## Error Handling Conventions

### Error Categories

```rust
pub enum AppError {
    // Client errors (4xx)
    BadRequest(String),
    Unauthorized(String),
    NotFound(String),
    Conflict(String),

    // Server errors (5xx)
    Internal(String),
    ServiceUnavailable(String),

    // External services
    OcrError(String),
    LlmError(String),
    StorageError(String),

    // Database
    Database(String),
}
```

### Error Response Format

```json
{
  "error": {
    "code": "OCR_TIMEOUT",
    "message": "OCR service timed out",
    "details": {
      "timeout": 30,
      "retries": 3
    },
    "request_id": "uuid"
  }
}
```

## Testing Conventions

### Test Organization

```
backend/src/
├── handlers/
│   ├── mod.rs
│   ├── analysis.rs
│   └── analysis_test.rs    # Handler tests
├── services/
│   ├── mod.rs
│   ├── ocr.rs
│   └── ocr_test.rs       # Service tests
└── tests/                # Integration tests
    └── api_tests.rs
```

### Test Naming

- Unit tests: `test_<function_name>`
- Integration tests: `test_<endpoint>_endpoint`
- Property tests: `prop_<property_name>`

```rust
#[test]
fn test_health_score_calculation() { }

#[tokio::test]
async fn test_upload_endpoint() { }
```

## Documentation Conventions

### Ownership and Updates

Follow the [documentation routing rules](../README.md#修改前先确定归属) before creating documents. Update the owning requirement/design for fixes, regressions, and optimizations; create a new pair only when no reasonable owner exists, and fill only a missing side when its counterpart exists. Workflow and approval gates live in [AGENTS.md](../../AGENTS.md), not in a second checklist here.

Historical feature documents have been removed with maintainer approval. Do not restore them in bulk or create an archive by default. Keep new documentation scoped to current work and separate approval, implementation, and verification status.

### API Documentation

Document intended request/response, authentication, errors, and compatibility in the owning technical design before implementation. Create a separate API reference only when needed and link it from that design; there is no current API reference to assume or restore. Shared types and handlers describe implementation reality, not automatic product approval.

### Code Documentation

- Public APIs: `///` doc comments
- Modules: `//!` module comments
- Examples: Code blocks in doc comments

## Deployment Conventions

### Environment Tiers

| Tier      | Purpose        | URL                                 |
| --------- | -------------- | ----------------------------------- |
| `dev`     | Development    | `dev-api.smart-ingredients.com`     |
| `staging` | Pre-production | `staging-api.smart-ingredients.com` |
| `prod`    | Production     | `api.smart-ingredients.com`         |

### Deployment Artifacts

- Backend: Docker image
- Frontend: Tauri installers (.dmg, .exe, .AppImage)
- Database: Migration scripts

### Rollback Procedure

1. Revert to previous Docker tag
2. Run database rollback migrations
3. Monitor error rates
4. Notify team of incident

## Monitoring Conventions

### Key Metrics

- Request rate and latency
- Error rate by endpoint
- OCR success rate
- LLM response time
- Database query performance

### Alerting Thresholds

| Metric               | Warning | Critical |
| -------------------- | ------- | -------- |
| Error rate           | > 1%    | > 5%     |
| P95 latency          | > 2s    | > 5s     |
| OCR failure          | > 5%    | > 10%    |
| Database connections | > 80%   | > 95%    |
