# Coding Standards - Smart Ingredients

## Rust General Standards

### Code Style

- Use `cargo fmt` for all formatting
- Follow standard Rust naming conventions:
  - Functions and methods: `snake_case`
  - Types and structs: `PascalCase`
  - Constants: `SCREAMING_SNAKE_CASE`
  - Modules: `snake_case`

### Linting

- Run `cargo clippy` before committing
- Address all clippy warnings or document exceptions
- Use `#![deny(warnings)]` in production code

### Error Handling

- Use `anyhow::Result<T>` for application errors
- Use `thiserror` for custom error types
- Provide context with `.context()` from anyhow
- Never use `unwrap()` or `expect()` in production code

```rust
// Good
use anyhow::{Context, Result};

fn process_image(image: &Image) -> Result<String> {
    let data = image
        .process()
        .context("Failed to process image")?;
    Ok(data)
}

// Bad
fn process_image(image: &Image) -> String {
    image.process().unwrap()
}
```

### Logging

- Use `tracing` for structured logging
- Include relevant context in log spans
- Use appropriate log levels:
  - `ERROR`: Errors that require attention
  - `WARN`: Unexpected but recoverable situations
  - `INFO`: Important events
  - `DEBUG`: Detailed debugging information

```rust
use tracing::{info, warn, error, instrument};

#[instrument(skip(image))]
async fn analyze_image(image: Image) -> Result<Analysis> {
    info!("Starting image analysis");
    // ...
}
```

## Frontend Standards (Tauri + Leptos)

### File Organization

```
frontend/src/
├── components/       # Reusable components
├── pages/          # Page-level components
├── services/       # API calls and business logic
├── stores/         # State management
├── types/          # Type definitions
└── utils/          # Utility functions
```

### Component Naming

- Component files: `kebab-case.rs`
- Component structs: `PascalCase`
- Props structs: `ComponentNameProps`

```rust
// file: components/image-uploader.rs
#[component]
pub fn ImageUploader() -> impl IntoView {
    // ...
}
```

### State Management

- Use Leptos signals for reactive state
- Keep state as local as possible
- Use `create_rw_signal` for read-write state
- Use `create_signal` for read-only derived state

```rust
let (count, set_count) = create_signal(0);
let is_even = move || count() % 2 == 0;
```

### API Calls

- Use `reqwest` for HTTP requests
- Handle loading and error states
- Implement request cancellation on component cleanup

## Backend Standards (Axum)

### File Organization

```
backend/src/
├── handlers/        # Request handlers
├── models/         # Database models
├── services/       # Business logic
├── routes/         # Route definitions
├── middleware/    # Custom middleware
├── db/            # Database connection and queries
├── errors/        # Error types
└── main.rs        # Application entry point
```

### Handler Functions

- Always async
- Return `Result<impl IntoResponse, AppError>`
- Use extractors for request parsing

```rust
use axum::{extract::Path, Json};

async fn get_analysis(
    Path(id): Path<Uuid>,
) -> Result<Json<AnalysisResponse>, AppError> {
    let analysis = fetch_analysis(id).await?;
    Ok(Json(analysis.into()))
}
```

### Database Operations

- Use SQLx with compile-time query checking
- Use transactions for multi-step operations
- Prefer prepared statements

```rust
use sqlx::{PgPool, query_as};

async fn get_analysis(pool: &PgPool, id: Uuid) -> Result<Analysis> {
    query_as!(
        Analysis,
        "SELECT * FROM analyses WHERE id = $1",
        id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::Database(e.into()))
}
```

### Route Organization

- Group related routes
- Use consistent path naming
- Apply middleware at appropriate levels

```rust
use axum::Router;

fn create_routes() -> Router {
    Router::new()
        .route("/api/v1/analysis/upload", post(upload_handler))
        .route("/api/v1/analysis/:id", get(get_handler))
        .layer(TraceLayer::new_for_http())
}
```

## Shared Types Standards

### Type Definitions

- Place all shared types in `shared/src/`
- Use `serde::{Serialize, Deserialize}`
- Keep API contracts versioned

```rust
// shared/src/types.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisRequest {
    pub image_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResponse {
    pub id: String,
    pub status: AnalysisStatus,
}
```

### Versioning

- Use semantic versioning for API changes
- Document breaking changes
- Maintain backward compatibility when possible

## Testing Standards

### Unit Tests

- Test business logic in isolation
- Mock external dependencies
- Aim for >80% code coverage

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_score_calculation() {
        let result = calculate_health_score(&ingredients);
        assert!(result >= 0 && result <= 100);
    }
}
```

### Integration Tests

- Test API endpoints
- Use test database
- Test error scenarios

```rust
#[tokio::test]
async fn test_upload_endpoint() {
    let app = create_test_app();
    let response = app
        .oneshot(Request::builder()
            .uri("/api/v1/analysis/upload")
            .body(Body::empty())
            .unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
```

## Documentation Standards

### Inline Documentation

- Document public APIs with `///`
- Use `#[doc(hidden)]` for internal items
- Include examples for complex functions

```rust
/// Analyzes a food ingredient image and returns health information.
///
/// # Arguments
///
/// * `image` - The image data to analyze
///
/// # Returns
///
/// A `Result` containing the `Analysis` or an error
///
/// # Example
///
/// ```
/// let analysis = analyze_image(image_data).await?;
/// println!("Health score: {}", analysis.health_score);
/// ```
pub async fn analyze_image(image: Vec<u8>) -> Result<Analysis> {
    // ...
}
```

### Module Documentation

- Add module-level documentation
- Explain the module's purpose
- Link to related modules

```rust
//! Handlers for analysis-related API endpoints.
//!
//! This module contains all the request handlers for the analysis API,
//! including image upload, status checking, and result retrieval.
```

## Git Commit Standards

### Commit Message Format

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

### Examples

```
feat(backend): add image upload endpoint

Implement multipart form handling for image uploads
with validation for file type and size.

Closes #123
```

```
fix(frontend): handle OCR timeout gracefully

Add retry logic and user feedback when OCR
service times out.

Fixes #456
```

## Performance Standards

### Database Queries

- Use indexes for frequently queried columns
- Avoid N+1 queries
- Use connection pooling

### Caching

- Cache expensive operations
- Set appropriate TTL values
- Invalidate cache on data changes

### Async Operations

- Use `tokio::spawn` for independent tasks
- Use `join!` for concurrent operations
- Avoid blocking the async runtime

## Security Standards

### Input Validation

- Validate all user inputs
- Sanitize database queries
- Limit file sizes and types

### Secrets Management

- Never commit secrets to git
- Use environment variables
- Rotate API keys regularly

### Dependencies

- Audit dependencies regularly
- Update to latest stable versions
- Review security advisories

```bash
cargo audit
cargo update
```
