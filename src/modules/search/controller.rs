//! `/api/search` route.

use std::sync::Arc;

use crate::app::AppState;

use super::service;
use axum::{Router, routing::get};

/// Declares `GET /search`.
///
/// Session-protected: results are filtered against the caller (blocks, room
/// membership), so the upstream needs an `x-user-id`.
pub fn search_routes() -> Router<Arc<AppState>> {
    Router::new().route("/search", get(service::search))
}
