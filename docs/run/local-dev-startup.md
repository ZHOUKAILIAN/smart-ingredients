# Local Development Startup

## Prerequisites

- Rust toolchain
- Node.js (for Tauri build tooling)
- `.env` file at repo root (copy from `.env.example`)

## Start Backend (local)

```bash
cd backend
cargo run
```

## Start Frontend (local)

```bash
cd frontend
cargo tauri dev
```

## Notes

- Backend default: `http://localhost:3000`
- If OCR is external, ensure `OCR_PADDLE_URL` is set in `.env`.
