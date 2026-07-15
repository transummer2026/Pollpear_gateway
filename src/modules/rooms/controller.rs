use std::sync::Arc;

use crate::app::AppState;

use super::service;
use axum::{
    Router,
    routing::{delete, get, patch, post},
};

pub fn rooms_routes() -> Router<Arc<AppState>> {
    Router::new().nest("/rooms", rooms())
}

fn rooms() -> Router<Arc<AppState>> {
    Router::new()
        .route("/new", post(service::create))
        .route("/{id}", patch(service::update))
        .route("/{id}", delete(service::remove))
        .route(
            "/{id}/participants/{userId}",
            post(service::add_participant),
        )
        .route(
            "/{id}/participants/{userId}",
            delete(service::remove_participant),
        )
        .route("/{id}/messages", get(service::get_messages))
        .route("/{id}/messages", post(service::create_message))
        .route(
            "/{id}/messages/{messageId}",
            delete(service::remove_message),
        )
}
