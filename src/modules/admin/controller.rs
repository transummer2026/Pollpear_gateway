use std::sync::Arc;

use crate::app::AppState;

use super::service;
use axum::{Router, routing::get};

pub fn admin_routes() -> Router<Arc<AppState>> {
    Router::new().nest("/admin", admin())
}

fn admin() -> Router<Arc<AppState>> {
    Router::new().route("/", get(service::dashboard))
}
