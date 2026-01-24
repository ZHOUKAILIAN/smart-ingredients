# Build stage
FROM docker.io/library/rust:1.88-slim AS builder

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        build-essential \
        libssl-dev \
        pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY backend/Cargo.toml backend/Cargo.toml
COPY shared/Cargo.toml shared/Cargo.toml

# Workspace placeholders
RUN mkdir -p frontend/src frontend/src-tauri/src backend/src shared/src \
    && printf '[package]\nname = "frontend-placeholder"\nversion = "0.1.0"\nedition = "2021"\n\n[lib]\npath = "src/lib.rs"\n' > frontend/Cargo.toml \
    && printf 'pub fn placeholder() {}\n' > frontend/src/lib.rs \
    && printf '[package]\nname = "frontend-tauri-placeholder"\nversion = "0.1.0"\nedition = "2021"\n\n[lib]\npath = "src/lib.rs"\n' > frontend/src-tauri/Cargo.toml \
    && printf 'pub fn placeholder() {}\n' > frontend/src-tauri/src/lib.rs \
    && printf 'fn main() {}\n' > backend/src/main.rs \
    && printf 'pub fn placeholder() {}\n' > shared/src/lib.rs

RUN cargo fetch

# Copy actual source code
COPY backend ./backend
COPY shared ./shared

# Build release binary
RUN cargo build --release -p backend

# Runtime stage
FROM docker.io/library/debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/backend /app/backend

ENV RUST_LOG=info

EXPOSE 3000

CMD ["/app/backend"]
