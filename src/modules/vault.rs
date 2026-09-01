//! Vault-backed configuration.
//!
//! The gateway holds no configuration of its own beyond the AppRole credentials
//! in [`crate::config::Config`]: everything else lives in Vault under
//! `secret/pollpear/*` and is read once at boot by
//! [`service::bootstrap`].
//!
//! The structs below mirror those secrets one-to-one. Every field is a
//! [`String`] because Vault KV v2 stores values as strings; the numeric ones are
//! parsed into [`crate::config::Settings`] during bootstrap, where a malformed
//! value fails startup rather than surfacing much later.

use serde::Deserialize;

pub mod service;

/// `secret/pollpear/app` — where the front end and the back end live.
#[derive(Deserialize)]
pub struct App {
    /// Browser origin, used as the single allowed CORS origin.
    pub frontend_url: String,
    /// Back-end base URL; every `*_service_url` is derived from it.
    pub backend_url: String,
}

/// `secret/pollpear/redis` — connection details for the shared Redis.
#[derive(Deserialize)]
pub struct Redis {
    pub host: String,
    pub port: String,
    pub password: String,
}

/// `secret/pollpear/session` — session policy.
#[derive(Deserialize)]
pub struct Session {
    /// Session lifetime in seconds.
    pub ttl: String,
}

/// `secret/pollpear/s3` — Garage object storage.
#[derive(Deserialize)]
pub struct S3 {
    /// Base URL `/api/uploads/*` is proxied to.
    pub url: String,
    /// `Host` header Garage expects for virtual-hosted bucket addressing.
    pub uploads_host: String,
}

/// `secret/pollpear/ws` — backend WebSocket endpoint.
#[derive(Deserialize)]
pub struct Ws {
    /// Path appended to the backend URL, scheme swapped to `ws`/`wss`.
    pub path: String,
}

/// `secret/pollpear/rate_limit` — `/api/v1` quota.
#[derive(Deserialize)]
pub struct RateLimit {
    /// Requests allowed per key and per window.
    pub limit: String,
    /// Window length in seconds.
    pub window_seconds: String,
}
