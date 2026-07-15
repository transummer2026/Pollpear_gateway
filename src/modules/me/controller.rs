use std::sync::Arc;

use crate::app::AppState;

use super::service;
use axum::{
    Router,
    routing::{delete, get, patch, post, put},
};

pub fn me_routes() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/me", me())
        .nest("/me", avatar())
        .nest("/me/settings", settings())
}

fn me() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(service::my_profile))
        .route("/", patch(service::edit_profile))
        .route("/", delete(service::remove))
}

fn avatar() -> Router<Arc<AppState>> {
    Router::new().route("/avatar", post(service::upload_avatar))
}

fn settings() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(service::my_settings))
        .route("/", patch(service::edit_settings))
        .route("/security", put(service::change_password))
}
