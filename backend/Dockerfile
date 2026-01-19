FROM docker.io/library/rust:1.88-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        clang \
        libclang-dev \
        libssl-dev \
        libopencv-dev \
        pkg-config \
        tesseract-ocr \
        tesseract-ocr-chi-sim \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY backend/Cargo.toml backend/Cargo.toml
COPY backend/src backend/src
COPY shared/Cargo.toml shared/Cargo.toml
COPY shared/src shared/src

# Workspace placeholders to satisfy Cargo when frontend sources are not in the build context.
RUN mkdir -p frontend/src frontend/src-tauri/src \
    && printf '[package]\nname = "frontend-placeholder"\nversion = "0.1.0"\nedition = "2021"\n\n[lib]\npath = "src/lib.rs"\n' > frontend/Cargo.toml \
    && printf 'pub fn placeholder() {}\n' > frontend/src/lib.rs \
    && printf '[package]\nname = "frontend-tauri-placeholder"\nversion = "0.1.0"\nedition = "2021"\n\n[lib]\npath = "src/lib.rs"\n' > frontend/src-tauri/Cargo.toml \
    && printf 'pub fn placeholder() {}\n' > frontend/src-tauri/src/lib.rs

RUN cargo fetch

COPY backend ./backend
COPY shared ./shared

ENV RUST_LOG=info

WORKDIR /app/backend

EXPOSE 3000

CMD ["cargo", "run", "-p", "backend"]
