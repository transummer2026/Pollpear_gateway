use std::sync::Arc;

use crate::app::AppState;

use super::service;
use axum::{Router, routing::post};

pub fn contact_routes() -> Router<Arc<AppState>> {
    Router::new().route("/contact", post(service::contact))
}
