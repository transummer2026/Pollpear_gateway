use std::sync::Arc;

use crate::app::AppState;

use super::service;
use axum::{Router, routing::get};

pub fn feed_routes() -> Router<Arc<AppState>> {
    Router::new().route("/feed", get(service::feed))
}
