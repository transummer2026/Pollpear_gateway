//! `/api/feed` route.

use std::sync::Arc;

use crate::app::AppState;

use super::service;
use axum::{Router, routing::get};

/// Declares `GET /feed`.
///
/// Session-protected: the feed is built from the caller's `x-user-id`.
pub fn feed_routes() -> Router<Arc<AppState>> {
    Router::new().route("/feed", get(service::feed))
}
