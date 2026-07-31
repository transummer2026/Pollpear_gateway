# syntax=docker/dockerfile:1
# escape=\
# check=error=true

FROM rust:1.96-slim AS chef

RUN cargo install cargo-chef --locked

WORKDIR /app

FROM chef AS planner

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

RUN cargo install cargo-watch --locked \
    && useradd -m rust

COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --recipe-path recipe.json

FROM builder AS dev

# hadolint ignore=DL3008
RUN apt-get update && apt-get install -y --no-install-recommends curl \
    && rm -rf /var/lib/apt/lists/* \
    && chown -R rust:rust /usr/local/cargo /app

USER rust

COPY --chown=rust:rust . .

ENV RUST_LOG=debug
CMD ["cargo", "watch", "-x", "run"]

FROM rust:1.96-alpine AS prod-chef

# hadolint ignore=DL3018
RUN apk add --no-cache musl-dev \
    && cargo install cargo-chef --locked

WORKDIR /app

FROM prod-chef AS prod-builder

COPY --from=planner /app/recipe.json recipe.json

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo chef cook --release --recipe-path recipe.json

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release --locked \
    && cp target/release/pollpear_gateway /usr/local/bin/pollpear_gateway

FROM alpine:3.21 AS prod

# hadolint ignore=DL3018
RUN apk add --no-cache curl \
    && adduser -D -H gateway

USER gateway

COPY --from=prod-builder /usr/local/bin/pollpear_gateway /usr/local/bin/pollpear_gateway

CMD ["pollpear_gateway"]
