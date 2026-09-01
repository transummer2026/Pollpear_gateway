//! `/api/v1` routes.

use std::sync::Arc;

use crate::app::AppState;
use crate::middleware::apikey::apikey;
use crate::middleware::ratelimit::ratelimit;

use super::service;
use axum::{Router, middleware, routing::any};

/// Declares the `/v1` subtree — the third-party API surface.
///
/// Separate from the session-protected routes: callers here are scripts and
/// integrations holding an API key, not browsers holding a cookie.
pub fn api_routes(app_state: &Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new().nest("/v1", v1(app_state))
}

/// Catch-all forwarding, with quota then key resolution.
///
/// [`any`] over `/` and `/{*path}`, so the gateway does not have to mirror the
/// upstream's route table: the API can gain endpoints without a change here.
/// Two routes because a wildcard does not match the empty remainder.
///
/// [`ratelimit`] is added last, making it the outer layer: quota is charged
/// before the key is resolved, so unknown keys are throttled too and the key
/// space cannot be enumerated.
fn v1(app_state: &Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/", any(service::proxy))
        .route("/{*path}", any(service::proxy))
        .route_layer(middleware::from_fn_with_state(app_state.clone(), apikey))
        .route_layer(middleware::from_fn_with_state(app_state.clone(), ratelimit))
}
