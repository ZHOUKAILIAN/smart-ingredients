# Claude Code Instructions - Smart Ingredients

## Project Overview

**Smart Ingredients** (食品配料表分析助手) - 智能食品配料表分析工具

- **Frontend**: Rust + Tauri + Leptos
- **Backend**: Rust + Axum + SQLx
- **Database**: PostgreSQL + Redis
- **OCR**: PaddleOCR / Tesseract
- **LLM**: DeepSeek / 智谱 AI

## Documentation-Driven Development Workflow

### MANDATORY: Documentation First

**CRITICAL RULE**: This project follows strict documentation-driven development. You MUST:

1. **NEVER write code without prior documentation**
2. **ALWAYS create/update docs before implementing features**
3. **ENSURE all design decisions are documented first**

### Workflow Steps

```
1. Document Requirements  →  docs/requirements/
2. Create Design Docs    →  docs/design/
3. Implement Code        →  Write code following docs
4. Update Documentation  →  Reflect any changes
```

### Documentation Structure

```
docs/
├── requirements/          # Feature requirements and specifications
├── design/              # Technical design documents
├── standards/           # Coding standards and conventions
└── analysis/           # Project analysis and research
```

## Before Writing Code

### Check if Documentation Exists

Before implementing any feature, verify that documentation exists:

1. **Requirements documented?** Check `docs/requirements/`
2. **Design documented?** Check `docs/design/`
3. **API documented?** Check `docs/api/` (if applicable)

If documentation is missing, **STOP and create it first**.

### Example: Adding a New Feature

**WRONG** (do not do this):
```
User: "Add user authentication"
Assistant: [Starts writing code immediately]
```

**CORRECT** (do this instead):
```
User: "Add user authentication"
Assistant: "I need to document this first. Let me create the requirements and design docs."
[Creates docs/requirements/auth.md]
[Creates docs/design/auth-flow.md]
[Then implements code]
```

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

### Rust Conventions

- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Follow Rust API guidelines
- Use `anyhow` for error handling
- Use `tracing` for logging

### Frontend (Leptos)

- Component files: `kebab-case.rs`
- Use Leptos signals for state
- Follow Leptos best practices

### Backend (Axum)

- Handler functions: `async fn`
- Use `axum::extract` for request parsing
- Return `Result<impl IntoResponse, AppError>`
- Use SQLx compile-time query checking

### Shared Types

- All shared types in `shared/src/`
- Use `serde::{Serialize, Deserialize}`
- Keep API contracts in sync

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
