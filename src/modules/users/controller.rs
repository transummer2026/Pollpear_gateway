//! `/api/users` routes.

use std::sync::Arc;

use crate::app::AppState;

use super::service;
use axum::{
    Router,
    routing::{delete, get, patch, post},
};

/// Declares the `/users` subtree.
///
/// Session-protected. Everything is addressed by `{id}`, in contrast with
/// [`crate::modules::me`], which is scoped to the caller.
pub fn users_routes() -> Router<Arc<AppState>> {
    Router::new().nest("/users", users())
}

/// Profiles, blocks, friendships and moderation actions.
///
/// Same-path/different-method pairs are deliberate: `/block`, `/friendship` and
/// `/ban` each model one relationship as a single resource, so its lifecycle
/// reads as `GET`/`POST`/`PATCH`/`DELETE` on one URL.
fn users() -> Router<Arc<AppState>> {
    Router::new()
        .route("/{id}", get(service::get_one))
        .route("/{id}", patch(service::update))
        .route("/{id}", delete(service::remove))
        .route("/{id}/report", post(service::report))
        .route("/{id}/block", get(service::get_block_status))
        .route("/{id}/block", post(service::block))
        .route("/{id}/block", delete(service::unblock))
        .route("/{id}/blocks", get(service::get_blocked_users))
        .route("/{id}/friendship", get(service::get_friendship))
        .route("/{id}/friendship", post(service::create_friend_request))
        .route("/{id}/friendship", patch(service::update_friend_request))
        .route("/{id}/friendship", delete(service::remove_friendship))
        .route("/{id}/friends", get(service::get_friends))
        .route("/{id}/friend-requests", get(service::get_friend_requests))
        .route("/{id}/ban", post(service::ban))
        .route("/{id}/ban", patch(service::update_ban))
        .route("/{id}/ban", delete(service::remove_ban))
}
