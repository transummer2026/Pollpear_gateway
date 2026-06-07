FROM rust:1.87-slim AS builder

WORKDIR /app

FROM alpine:3.21 AS prod

WORKDIR /app