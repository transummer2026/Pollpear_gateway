//! `/api/polls` routes.

use std::sync::Arc;

use crate::app::AppState;

use super::service;
use axum::{
    Router,
    routing::{delete, patch, post, get},
};

/// Declares the `/polls` subtree.
///
/// Session-protected: voting and moderation both need the caller's identity.
pub fn polls_routes() -> Router<Arc<AppState>> {
    Router::new().nest("/polls", polls())
}

/// Polls, their options and the caller's vote.
///
/// A vote is not addressed by id: `/{id}/vote` always means *the caller's* vote
/// on that poll, resolved upstream from `x-user-id`. Listing and reading share
/// [`service::read`], which distinguishes them from the forwarded path.
fn polls() -> Router<Arc<AppState>> {
    Router::new()
        .route("/new", post(service::create))
        .route("/", get(service::read))
        .route("/{id}", get(service::read))
        .route("/{id}", patch(service::update))
        .route("/{id}", delete(service::remove))
        .route("/{id}/report", post(service::report))
        .route(
            "/{id}/options/{optionId}/report",
            post(service::report_option),
        )
        .route("/{id}/options/{optionId}", delete(service::remove_option))
        .route("/{id}/vote", post(service::create_vote))
        .route("/{id}/vote", patch(service::update_vote))
        .route("/{id}/vote", delete(service::remove_vote))
        .route("/{id}/vote/report", post(service::report_vote))
}
