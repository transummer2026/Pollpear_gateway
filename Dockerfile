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

RUN chown -R rust:rust /usr/local/cargo

USER rust

COPY --chown=rust:rust . .

ENV RUST_LOG=tower_http=trace
CMD ["cargo", "watch", "-x", "run"]

FROM alpine:3.21 AS prod
