//! `/api/me` routes.

use std::sync::Arc;

use crate::app::AppState;

use super::service;
use axum::{
    Router,
    routing::{delete, get, patch, post, put},
};

/// Declares the `/me` subtree — everything scoped to the caller.
///
/// No route takes a user id: the subject is always the `x-user-id` resolved by
/// the session middleware, which is what separates these endpoints from
/// [`crate::modules::users`], where the id is in the path.
pub fn me_routes() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/me", me())
        .nest("/me", avatar())
        .nest("/me/notifications", notifications())
        .nest("/me/settings/security", security())
}

/// Notifications: list, mark all read, dismiss one.
fn notifications() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(service::get_notifications))
        .route("/read", patch(service::read_notifications))
        .route("/{id}", delete(service::dismiss_notification))
}

/// Profile: read, edit, delete the account, and list received room invitations.
fn me() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(service::my_profile))
        .route("/", patch(service::edit_profile))
        .route("/", delete(service::remove))
        .route("/invitations", get(service::get_room_invitations))
}

/// Avatar upload.
///
/// Its own nest so it sits at `/me/avatar` rather than under the profile
/// router, whose routes are all mounted at `/`.
fn avatar() -> Router<Arc<AppState>> {
    Router::new().route("/avatar", post(service::upload_avatar))
}

/// Security: password, and the API keys used on `/api/v1`.
///
/// `PUT` for the password because it replaces the credential outright, where
/// the `PATCH` endpoints above amend a resource.
fn security() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", put(service::change_password))
        .route("/apikey", get(service::read_apikey))
        .route("/apikey", post(service::create_apikey))
        .route("/apikey", patch(service::rename_apikey))
        .route("/apikey", delete(service::revoke_apikey))
}
