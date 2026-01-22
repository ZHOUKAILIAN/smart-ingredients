# Claude Code Instructions - Smart Ingredients

## Project Overview

**Smart Ingredients** (食品配料表分析助手) - 智能食品配料表分析工具

- **Frontend**: Rust + Tauri + Leptos
- **Backend**: Rust + Axum + SQLx
- **Database**: PostgreSQL + Redis
- **OCR**: PaddleOCR / Tesseract
- **LLM**: DeepSeek / 智谱 AI

---

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
│   └── XXX-feature-requirements.md
├── design/               # Technical design documents
│   ├── technical-design.md
│   ├── ui-design.md
│   └── XXX-feature-technical-plan.md
├── api/                  # API documentation
│   └── api-reference.md
├── standards/            # Coding standards and conventions
│   ├── coding-standards.md
│   ├── error-handling-standards.md
│   ├── project-conventions.md
│   ├── requirements-template.md
│   └── technical-design-template.md
├── analysis/             # Project analysis and research
│   └── project-analysis.md
└── run/                  # Operational guides
    ├── backend-startup.md
    └── integration-testing.md
```

---

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
[Creates docs/design/005-authentication-technical-plan.md]
[Creates docs/design/auth-flow.md]
[Updates docs/api/api-reference.md with auth endpoints]
```

**Why correct**: Requirements documented first, design decisions made, API contracts defined before any code.

---

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

---

## Coding Standards

**IMPORTANT**: All implementation details are in `docs/standards/`. You MUST read these before writing code:

- **`coding-standards.md`** - General Rust coding conventions, formatting, linting
- **`error-handling-standards.md`** - Error handling rules, patterns, and decision guides
- **`project-conventions.md`** - Project-specific patterns and conventions

### Quick Reference

| Topic | Rule | See Details |
|-------|------|------------|
| **Formatting** | Use `cargo fmt` before committing | `coding-standards.md` |
| **Linting** | Use `cargo clippy` and fix all warnings | `coding-standards.md` |
| **Error Handling** | Backend: `Result<impl IntoResponse, AppError>`<br>Service: `anyhow::Result<T>`<br>Frontend: `Result<T, ErrorInfo>` | `error-handling-standards.md` |
| **Logging** | Use `tracing` macros (`error!`, `warn!`, `info!`, `debug!`) | `coding-standards.md` |
| **Async/Await** | Entry: `#[tokio::main]`<br>Handlers: `async fn handler() -> Result<...>` | `coding-standards.md` |
| **File Naming** | Frontend components: `kebab-case.rs`<br>Backend modules: `snake_case.rs` | `project-conventions.md` |
| **Database** | Use SQLx with `query_as!` for type safety<br>Migrations: `YYYYMMDDHHMMSS_description.sql` | `coding-standards.md` |
| **Shared Types** | Always derive `Serialize` + `Deserialize`<br>Location: `shared/src/` | `project-conventions.md` |

---

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

---

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

---

## Testing Strategy

See `docs/standards/coding-standards.md` for detailed testing patterns.

- Unit tests for business logic
- Integration tests for API endpoints
- E2E tests for critical user flows

---

## Deployment

- **Frontend**: Tauri installers (.dmg, .exe, .AppImage)
- **Backend**: Docker + K8s
- **Database**: RDS PostgreSQL
- **Cache**: Redis Cluster

---

## Important Notes

- This is a new project - establish patterns early
- Document decisions as you make them
- Keep docs in sync with code changes
- **ALWAYS read `docs/standards/` before implementing features**
