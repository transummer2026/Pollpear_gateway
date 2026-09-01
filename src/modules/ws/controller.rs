//! `/api/ws` routes.

use std::sync::Arc;

use crate::app::AppState;

use super::service;
use axum::{Router, routing::get};

/// Declares the WebSocket handshake endpoint.
///
/// Registered with and without a trailing slash because a redirect would break
/// the upgrade: browsers do not follow a `3xx` on a WebSocket handshake.
///
/// [`get`] and not a WebSocket-specific method: the protocol upgrade *is* a
/// `GET` carrying `Upgrade: websocket`, handled by the extractor in the handler.
pub fn ws_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/ws", get(service::handshake))
        .route("/ws/", get(service::handshake))
}
