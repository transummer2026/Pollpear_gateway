//! `/health` route.

use super::service;
use axum::{Router, routing::get};

/// Declares `GET /health`.
///
/// Generic over the state type and merged outside the `/api` nest, so the probe
/// needs no [`crate::app::AppState`] and stays reachable whatever happens to the
/// API surface.
pub fn healthcheck_routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new().route("/health", get(service::health))
}
