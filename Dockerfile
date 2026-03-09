FROM rust:1.86-bookworm AS builder

WORKDIR /app

# Copy manifests first to optimize Docker layer cache.

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update \
    && apt-get install --no-install-recommends -y wget \
    && rm -rf /var/lib/apt/lists/*

RUN useradd --system --no-create-home appuser

COPY --from=builder /app/target/release/why-is-it-green-api /usr/local/bin/why-is-it-green-api

USER appuser

EXPOSE 3000

CMD ["why-is-it-green-api"]
