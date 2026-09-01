//! Runtime configuration.
//!
//! Split in three, by source and lifetime:
//!
//! - [`Config`]: the few values needed *before* Vault can be reached, read from
//!   the environment.
//! - [`ServiceUrl`]: every upstream address, read from Vault at boot.
//! - [`Settings`]: tunable policy values (TTL, quotas), also read from Vault.
//!
//! [`ServiceUrl`] and [`Settings`] are resolved once at startup and then live
//! in the shared [`crate::app::AppState`]; nothing re-reads Vault at runtime.

use std::path::Path;

/// Vault credentials — the only configuration taken from the environment.
///
/// Everything else is a Vault secret, so these three values are the root of
/// trust for the whole gateway configuration.
#[derive(Clone)]
pub struct Config {
    /// Vault server address (`VAULT_ADDR`).
    pub vault_url: String,
    /// AppRole role id (`ROLE_ID`).
    pub vault_role: String,
    /// AppRole secret id (`SECRET_ID`).
    pub vault_secret: String,
}

/// Upstream addresses the gateway forwards to.
///
/// The per-domain `*_service_url` fields all point at the same monolith today,
/// but each route reads its own field — splitting a domain out to its own
/// service is a change in [`crate::modules::vault::service::bootstrap`] only,
/// with no route to touch.
#[derive(Clone)]
pub struct ServiceUrl {
    /// Upstream serving the public `/api/v1` API-key surface.
    pub api_service_url: String,
    /// Upstream serving `/api/auth` and `/api/auth/oauth`.
    pub auth_service_url: String,
    /// Upstream serving `/api/feed`.
    pub feed_service_url: String,
    /// Upstream serving `/api/me`.
    pub me_service_url: String,
    /// Upstream serving `/api/polls`.
    pub polls_service_url: String,
    /// Upstream serving `/api/rooms`.
    pub rooms_service_url: String,
    /// Upstream serving `/api/search`.
    pub search_service_url: String,
    /// Upstream serving `/api/users`.
    pub users_service_url: String,
    /// Backend WebSocket endpoint, `ws(s)://…/<path>/`, derived from the
    /// backend URL plus the `pollpear/ws` secret.
    pub ws_url: String,
    /// Single browser origin allowed by CORS.
    pub frontend_url: String,
    /// Redis connection string, password included.
    pub redis_url: String,
    /// Garage (S3) base URL used to serve `/api/uploads/*`.
    pub garage_url: String,
    /// `Host` header Garage expects for virtual-hosted bucket addressing.
    pub garage_uploads_host: String,
}

/// Policy values applied by the middlewares.
#[derive(Clone)]
pub struct Settings {
    /// Session lifetime in seconds, re-applied on refresh.
    pub session_ttl: i64,
    /// Maximum `/api/v1` requests allowed per API key and per window.
    pub rate_limit: u64,
    /// Length of the rate-limit window, in seconds.
    pub rate_limit_window_seconds: i64,
}

impl Config {
    /// Loads `../.env` when present.
    ///
    /// Sibling path because the gateway runs from its own directory inside the
    /// repo, next to the shared `.env`. Absent in production, where the
    /// variables come from the container environment — hence the ignored error.
    fn init_dotenvy() {
        dotenvy::from_path(Path::new("../.env")).ok();
    }

    /// Reads the Vault credentials from the environment.
    ///
    /// A missing variable yields a placeholder string rather than a panic; the
    /// failure then surfaces as a Vault login error during
    /// [`crate::modules::vault::service::bootstrap`], which reports which step
    /// actually failed.
    pub fn from_env() -> Self {
        Self::init_dotenvy();
        Self {
            vault_url: dotenvy::var("VAULT_ADDR")
                .unwrap_or_else(|_| "VAULT_ADDR is missing".to_string()),
            vault_role: dotenvy::var("ROLE_ID")
                .unwrap_or_else(|_| "ROLE_ID is missing".to_string()),
            vault_secret: dotenvy::var("SECRET_ID")
                .unwrap_or_else(|_| "SECRET_ID is missing".to_string())
                .to_string(),
        }
    }
}
