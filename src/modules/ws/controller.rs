use std::sync::Arc;

use crate::app::AppState;

use super::service;
use axum::{Router, routing::get};

pub fn ws_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/ws", get(service::handshake))
        .route("/ws/", get(service::handshake))
}
