# syntax=docker/dockerfile:1
# escape=\
# check=error=true

FROM rust:1.96-slim AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs \
    && cargo build --release \
    && rm -rf src

RUN cargo install cargo-watch --locked \
    && useradd -m rust

FROM builder AS dev

RUN chown -R rust:rust /usr/local/cargo /app

USER rust

COPY --chown=rust:rust . .

ENV RUST_LOG=debug
CMD ["cargo", "watch", "-x", "run"]

FROM rust:1.96-alpine AS prod-builder

# hadolint ignore=DL3018
RUN apk add --no-cache musl-dev

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release --locked \
    && cp target/release/pollpear_gateway /usr/local/bin/pollpear_gateway

FROM alpine:3.21 AS prod

RUN adduser -D -H gateway
USER gateway
COPY --from=prod-builder /usr/local/bin/pollpear_gateway /usr/local/bin/pollpear_gateway

CMD ["pollpear_gateway"]