# -----------------------------
# Stage 1 — Build
# -----------------------------

FROM rust:1.86-bookworm AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

# -----------------------------
# Stage 2 — Runtime
# -----------------------------

FROM debian:bookworm-slim

WORKDIR /app

ARG BUILD_NUMBER=1
ARG GIT_SHA=?

ENV BUILD_NUMBER=$BUILD_NUMBER
ENV GIT_SHA=$GIT_SHA

RUN apt-get update \
    && apt-get install --no-install-recommends -y wget \
    && rm -rf /var/lib/apt/lists/*

RUN useradd --system --no-create-home appuser

COPY --from=builder /app/target/release/why-is-it-green-api /usr/local/bin/why-is-it-green-api

USER appuser

EXPOSE 3000

CMD ["why-is-it-green-api"]
