FROM rust:1.88-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        libssl-dev \
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
COPY frontend/Cargo.toml frontend/Cargo.toml
COPY frontend/src frontend/src

RUN cargo fetch

COPY backend ./backend
COPY shared ./shared
COPY frontend ./frontend

ENV RUST_LOG=info

WORKDIR /app/backend

EXPOSE 3000

CMD ["cargo", "run", "-p", "backend"]
