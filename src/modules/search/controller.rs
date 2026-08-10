use std::sync::Arc;

use crate::app::AppState;

use super::service;
use axum::{Router, routing::get};

pub fn search_routes() -> Router<Arc<AppState>> {
    Router::new().route("/search", get(service::search))
}
