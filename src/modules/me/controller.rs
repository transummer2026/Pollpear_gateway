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
        .nest("/me/notifications", notifications())
        .nest("/me/settings", settings())
        .nest("/me/settings/security", security())
}

fn notifications() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(service::get_notifications))
        .route("/read", patch(service::read_notifications))
        .route("/{id}", delete(service::dismiss_notification))
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
}

fn security() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", put(service::change_password))
        .route("/apikey", get(service::read_apikey))
        .route("/apikey", post(service::create_apikey))
        .route("/apikey", patch(service::rename_apikey))
        .route("/apikey", delete(service::revoke_apikey))
}
