//! `/api/rooms` routes.

use std::sync::Arc;

use crate::app::AppState;

use super::service;
use axum::{
    Router,
    routing::{delete, get, patch, post},
};

/// Declares the `/rooms` subtree.
///
/// Session-protected: room access depends entirely on who the caller is.
pub fn rooms_routes() -> Router<Arc<AppState>> {
    Router::new().nest("/rooms", rooms())
}

/// Rooms and their three sub-resources: invitations, participants, messages.
///
/// `/feed` is declared after `/{id}` but still wins: axum matches the literal
/// segment before the parameter, so ordering here is presentational only.
///
/// An invitation is keyed by `{id}/{userId}` — one per room and user — which is
/// why the whole invitation lifecycle hangs off that single path.
fn rooms() -> Router<Arc<AppState>> {
    Router::new()
        .route("/new", post(service::create))
        .route("/", get(service::read))
        .route("/{id}", get(service::read_one))
        .route("/feed", get(service::read_many))
        .route("/{id}", patch(service::update))
        .route("/{id}", delete(service::remove))
        .route("/{id}/invitations", get(service::get_invitations))
        .route("/{id}/invitations/{userId}", get(service::get_invitation))
        .route(
            "/{id}/invitations/{userId}",
            post(service::create_invitation),
        )
        .route(
            "/{id}/invitations/{userId}",
            patch(service::respond_invitation),
        )
        .route(
            "/{id}/invitations/{userId}",
            delete(service::remove_invitation),
        )
        .route("/{id}/participants", post(service::add_participant))
        .route(
            "/{id}/participants/{userId}",
            delete(service::remove_participant),
        )
        .route("/{id}/messages", get(service::get_messages))
        .route("/{id}/messages", post(service::create_message))
}
