//! `/api/uploads` route.

use std::sync::Arc;

use crate::app::AppState;

use super::service;
use axum::{Router, routing::get};

/// Declares `GET /uploads/{*path}`.
///
/// Public and read-only: avatars and poll images are served to anyone, and
/// uploading goes through the owning service instead (for instance
/// `POST /api/me/avatar`), which validates and stores the object itself.
pub fn uploads_routes() -> Router<Arc<AppState>> {
    Router::new().route("/uploads/{*path}", get(service::upload))
}
