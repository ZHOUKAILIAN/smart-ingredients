# Claude Code Instructions - Smart Ingredients

## Project Overview

**Smart Ingredients** (食品配料表分析助手) - 智能食品配料表分析工具

- **Frontend**: Rust + Tauri + Leptos
- **Backend**: Rust + Axum + SQLx
- **Database**: PostgreSQL + Redis
- **OCR**: PaddleOCR / Tesseract
- **LLM**: DeepSeek / 智谱 AI

## Documentation-Driven Development Workflow

### 🚨 MANDATORY: Documentation First - NO EXCEPTIONS 🚨

**ABSOLUTE RULE**: This project enforces strict documentation-driven development.

#### You MUST:

1. ✅ **NEVER write code without prior documentation**
2. ✅ **ALWAYS create/update docs before implementing features**
3. ✅ **ENSURE all design decisions are documented first**
4. ✅ **STOP and ask if documentation is unclear or missing**
5. ✅ **UPDATE docs immediately when implementation deviates from design**

#### You MUST NOT:

1. ❌ **Start coding without reading relevant docs first**
2. ❌ **Skip documentation "to save time"**
3. ❌ **Assume implementation details not in docs**
4. ❌ **Make architectural decisions without documenting them**
5. ❌ **Proceed with ambiguous requirements**

### Workflow Steps (MANDATORY SEQUENCE)

```
┌──────────────────────────────────────────────────────────┐
│ 1. READ EXISTING DOCS                                    │
│    - Check docs/requirements/ for feature specs          │
│    - Check docs/design/ for technical design             │
│    - Check docs/standards/ for coding conventions        │
└──────────────────────────────────────────────────────────┘
                         ↓
┌──────────────────────────────────────────────────────────┐
│ 2. VERIFY COMPLETENESS                                   │
│    - Requirements clear and complete?                    │
│    - Design decisions documented?                        │
│    - API contracts defined?                              │
│    - If NO → Create missing documentation                │
└──────────────────────────────────────────────────────────┘
                         ↓
┌──────────────────────────────────────────────────────────┐
│ 3. ASK QUESTIONS (if needed)                             │
│    - Clarify ambiguous requirements                      │
│    - Confirm architectural choices                       │
│    - Validate assumptions                                │
└──────────────────────────────────────────────────────────┘
                         ↓
┌──────────────────────────────────────────────────────────┐
│ 4. DOCUMENT DESIGN (if not exists)                       │
│    - Create technical design doc                         │
│    - Define data structures                              │
│    - Specify API contracts                               │
│    - Document error handling                             │
└──────────────────────────────────────────────────────────┘
                         ↓
┌──────────────────────────────────────────────────────────┐
│ 5. IMPLEMENT CODE                                        │
│    - Follow documented design exactly                    │
│    - Use patterns from docs/standards/                   │
│    - Reference doc sections in code comments             │
└──────────────────────────────────────────────────────────┘
                         ↓
┌──────────────────────────────────────────────────────────┐
│ 6. UPDATE DOCS (if implementation differs)               │
│    - Document deviations from design                     │
│    - Update API specs if changed                         │
│    - Add lessons learned                                 │
└──────────────────────────────────────────────────────────┘
```

### Documentation Structure

```
docs/
├── requirements/          # Feature requirements and specifications
│   ├── XXX-feature-requirements.md
│   └── XXX-feature-technical-plan.md
├── design/               # Technical design documents
│   ├── technical-design.md
│   └── ui-design.md
├── api/                  # API documentation
│   └── api-reference.md
├── standards/            # Coding standards and conventions
│   ├── coding-standards.md
│   ├── project-conventions.md
│   ├── requirements-template.md
│   └── technical-design-template.md
├── analysis/             # Project analysis and research
│   └── project-analysis.md
└── run/                  # Operational guides
    ├── backend-startup.md
    └── integration-testing.md
```

## Before Writing Code - Mandatory Checklist

### ✅ Pre-Implementation Checklist

Before writing ANY code, you MUST verify:

- [ ] **Requirements exist** in `docs/requirements/`
  - Feature purpose clearly stated
  - User stories or use cases defined
  - Success criteria specified

- [ ] **Design documented** in `docs/design/`
  - Architecture decisions explained
  - Data structures defined
  - API contracts specified
  - Error handling strategy documented

- [ ] **Standards reviewed** in `docs/standards/`
  - Coding conventions understood
  - Project patterns identified
  - Naming conventions clear

- [ ] **API contracts** in `docs/api/` (if applicable)
  - Request/response formats defined
  - Error responses documented
  - Authentication requirements clear

**If ANY checkbox is unchecked → STOP and create missing documentation**

### Example: Adding a New Feature

#### ❌ WRONG (do not do this):
```
User: "Add user authentication"
Assistant: "I'll add JWT authentication to the backend..."
[Starts writing code immediately]
```

**Why wrong**: No requirements documented, no design decisions made, no API contracts defined.

#### ✅ CORRECT (do this instead):
```
User: "Add user authentication"
Assistant: "I need to document this feature first. Let me check existing docs..."
[Reads docs/requirements/ and docs/design/]
Assistant: "I don't see authentication requirements. Let me create the documentation."

[Creates docs/requirements/005-authentication-requirements.md]
[Creates docs/requirements/005-authentication-technical-plan.md]
[Creates docs/design/auth-flow.md]
[Updates docs/api/api-reference.md with auth endpoints]
```

**Why correct**: Requirements documented first, design decisions made, API contracts defined before any code.

## Project Architecture

### Monorepo Structure

```
smart-ingredients/
├── frontend/           # Rust + Tauri + Leptos
├── backend/            # Rust + Axum + SQLx
├── shared/             # Shared types (workspace member)
├── docs/               # Documentation
├── scripts/            # Development scripts
└── Cargo.toml         # Workspace config
```

### Tech Stack Details

| Layer | Technology |
|-------|-----------|
| Frontend Framework | Tauri 2.x + Leptos 0.7.x |
| Backend Framework | Axum 0.7.x |
| Database | PostgreSQL 16.x + SQLx |
| Cache | Redis 7.x |
| OCR | PaddleOCR (Python) / Tesseract (Rust) |
| LLM | DeepSeek / 智谱 AI |

## Coding Standards

**IMPORTANT**: See `docs/standards/coding-standards.md` and `docs/standards/project-conventions.md` for complete details.

### Rust Conventions

- **Formatting**: Use `cargo fmt` before committing
- **Linting**: Use `cargo clippy` and fix all warnings
- **API Guidelines**: Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- **Error Handling**: Use `anyhow::Result` in services, `thiserror` for custom errors
- **Logging**: Use `tracing` macros (`info!`, `warn!`, `error!`, `debug!`)

### Error Handling Patterns

```rust
// Backend: Custom error types with thiserror
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("validation error: {0}")]
    Validation(String),
}

// Services: Use anyhow::Result
pub async fn process_data(input: &str) -> anyhow::Result<Output> {
    // ... implementation
}

// Handlers: Convert to AppError
pub async fn handler() -> Result<impl IntoResponse, AppError> {
    let result = service.process().await?;
    Ok(Json(result))
}

// Frontend: Use Result<T, String>
pub async fn fetch_data() -> Result<Data, String> {
    // ... implementation
}
```

### Async/Await Patterns

```rust
// Entry point
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // ... setup
}

// Handlers and services
pub async fn handler() -> Result<impl IntoResponse, AppError> {
    // ... implementation
}

// Trait methods
#[async_trait]
pub trait Service {
    async fn process(&self) -> anyhow::Result<Output>;
}

// CPU-bound work
tokio::task::spawn_blocking(move || {
    // Expensive computation
}).await?
```

### Frontend (Leptos)

**Component Structure**:
```rust
use leptos::prelude::*;

#[component]
pub fn MyComponent() -> impl IntoView {
    // 1. Get context/state
    let state = use_context::<AppState>()
        .expect("AppState not found");

    // 2. Create local signals
    let (count, set_count) = create_signal(0);

    // 3. Define event handlers
    let on_click = move |_| {
        set_count.update(|n| *n += 1);
    };

    // 4. Return view
    view! {
        <div>
            <button on:click=on_click>
                "Count: " {count}
            </button>
        </div>
    }
}
```

**State Management**:
```rust
// 1. Define state struct
#[derive(Clone)]
pub struct AppState {
    pub data: RwSignal<Option<Data>>,
    pub error: RwSignal<Option<String>>,
}

// 2. Provide at root
#[component]
pub fn App() -> impl IntoView {
    provide_context(AppState {
        data: create_rw_signal(None),
        error: create_rw_signal(None),
    });
    // ... rest of app
}

// 3. Consume in components
let state = use_context::<AppState>()
    .expect("AppState not found");
```

**File Naming**: Use `kebab-case.rs` for component files

### Backend (Axum)

**Handler Pattern**:
```rust
use axum::{
    extract::{State, Path, Json},
    response::{IntoResponse, Response},
};

pub async fn handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<CreateRequest>,
) -> Result<impl IntoResponse, AppError> {
    // 1. Validate input
    payload.validate()?;

    // 2. Call service
    let result = state.service.process(id, payload).await?;

    // 3. Return response
    Ok(Json(result))
}
```

**Router Pattern**:
```rust
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/items", get(list_items).post(create_item))
        .route("/items/:id", get(get_item).put(update_item).delete(delete_item))
}
```

**State Pattern**:
```rust
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub redis: ConnectionManager,
    pub config: Arc<Config>,
}
```

### Database (SQLx)

**Query Pattern**:
```rust
// Use FromRow derive
#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

// Raw SQL queries
let user = sqlx::query_as::<_, User>(
    "SELECT id, name, created_at FROM users WHERE id = $1"
)
.bind(user_id)
.fetch_one(&pool)
.await?;

// Transactions
let mut tx = pool.begin().await?;
sqlx::query("INSERT INTO ...").execute(&mut *tx).await?;
sqlx::query("UPDATE ...").execute(&mut *tx).await?;
tx.commit().await?;
```

**Migration Naming**: `YYYYMMDDHHMMSS_description.sql`

### Shared Types

**Location**: All shared types in `shared/src/`

**Pattern**:
```rust
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Request/Response types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemResponse {
    pub id: Uuid,
    pub name: String,
    pub created_at: String,
}
```

**Requirements**:
- Always derive `Serialize` + `Deserialize`
- Add documentation comments
- Keep in sync between frontend and backend
- Use `Option<T>` for nullable fields

## Development Commands

```bash
# Run backend
cd backend && cargo run

# Run frontend
cd frontend && cargo tauri dev

# Run tests
cargo test

# Format code
cargo fmt

# Lint code
cargo clippy
```

## Environment Variables

```bash
# Backend
DATABASE_URL=postgresql://...
REDIS_URL=redis://...
OCR_SERVICE_URL=http://...
LLM_API_KEY=...

# Frontend
VITE_API_URL=http://localhost:3000
```

## Testing Strategy

- Unit tests for business logic
- Integration tests for API endpoints
- E2E tests for critical user flows

## Deployment

- **Frontend**: Tauri installers (.dmg, .exe, .AppImage)
- **Backend**: Docker + K8s
- **Database**: RDS PostgreSQL
- **Cache**: Redis Cluster

## Important Notes

- This is a new project - establish patterns early
- Document decisions as you make them
- Keep docs in sync with code changes
- Review TECH-STACK.md for detailed technology choices
