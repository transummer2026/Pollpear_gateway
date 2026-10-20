# Pollpear_gateway

The Rust edge in front of `/api`. Every request to the Pollpear backend passes through
here first: the gateway resolves the session, authenticates API keys, enforces rate
limits, strips path prefixes, and proxies WebSocket upgrades — so those cross-cutting
concerns live in one fast, memory-safe place instead of being spread across services.

Part of **[Pollpear](https://github.com/transummer2026/Pollpear)**, a real-time collaborative polling platform.

## Stack

Rust (edition 2024) · [axum](https://github.com/tokio-rs/axum) 0.8 · hyper · tower /
tower-http · tokio-tungstenite for WebSocket proxying · `vaultrs` to read secrets from
HashiCorp Vault · Redis for session and rate-limit state.

## Layout

| Path | Role |
| :--- | :--- |
| `src/app.rs` | Route table — the public, protected and API-key surfaces |
| `src/middleware/` | `session`, `apikey`, `ratelimit`, `cors`, `strip` |
| `src/modules/` | One proxy module per backend domain (`auth`, `me`, `polls`, `rooms`, `users`, `feed`, `search`, `uploads`, `ws`, …) plus `vault`, `redis` and `healthcheck` |
| `src/proxy.rs` | The forwarding layer itself |
| `src/server.rs` | Startup, Vault bootstrap with retry, graceful shutdown |

Routing is deliberately explicit: `/api/auth/**` and `/api/uploads/*` are public,
`/api/{feed,me,polls,rooms,search,users,ws}` require a session cookie, and `/api/v1/**`
takes an API key with rate limiting. `/health` sits outside `/api` with no auth.

## Running it

The gateway expects Vault and Redis, so run it through the stack rather than alone:

```sh
git clone --recursive https://github.com/transummer2026/Pollpear.git
cd Pollpear && make up_prod
```

Standalone checks: `cargo check`, `cargo clippy`, `cargo test`.

Full setup instructions live in the [superproject README](https://github.com/transummer2026/Pollpear#readme).
