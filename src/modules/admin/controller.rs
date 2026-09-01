//! `/api/admin` routes.

use std::sync::Arc;

use crate::app::AppState;

use super::service;
use axum::{Router, routing::get};

/// Declares the `/admin` subtree.
///
/// Session-protected like every other route in that group; the admin role
/// itself is checked by the upstream service, which owns that information.
pub fn admin_routes() -> Router<Arc<AppState>> {
    Router::new().nest("/admin", admin())
}

/// `GET /` — the dashboard.
fn admin() -> Router<Arc<AppState>> {
    Router::new().route("/", get(service::dashboard))
}
