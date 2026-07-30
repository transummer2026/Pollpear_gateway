use std::sync::Arc;

use crate::app::AppState;

use super::service;
use axum::{Router, routing::get};

pub fn uploads_routes() -> Router<Arc<AppState>> {
    Router::new().route("/uploads/{*path}", get(service::upload))
}
