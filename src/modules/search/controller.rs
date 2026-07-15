use std::sync::Arc;

use crate::app::AppState;

use super::service;
use axum::{Router, routing::post};

pub fn search_routes() -> Router<Arc<AppState>> {
    Router::new().route("/search", post(service::search))
}
