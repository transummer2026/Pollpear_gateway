use std::sync::Arc;

use crate::app::AppState;

use super::service;
use axum::{
    Router,
    routing::{delete, patch, post},
};

pub fn users_routes() -> Router<Arc<AppState>> {
    Router::new().nest("/users", rooms())
}

fn rooms() -> Router<Arc<AppState>> {
    Router::new()
        .route("/{id}", patch(service::update))
        .route("/{id}", delete(service::remove))
        .route("/{id}/report", post(service::report))
        .route("{id}/block", post(service::block))
        .route("{id}/friendship", post(service::create_friend_request))
        .route("{id}/friendship", patch(service::update_friend_request))
        .route("{id}/firendship", delete(service::remove_friendship))
        .route("{id}/ban", post(service::ban))
        .route("{id}/ban", patch(service::update_ban))
        .route("{id}/ban", delete(service::remove_ban))
}
