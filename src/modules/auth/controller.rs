//! `/api/auth` routes.

use std::sync::Arc;

use crate::app::AppState;

use super::service;
use axum::{Router, routing::get, routing::post};

/// Declares the `/auth` and `/auth/oauth` subtrees.
///
/// Public — the session that protects everything else is created here, so these
/// routes cannot require one.
pub fn auth_routes() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/auth", auth())
        .nest("/auth/oauth", oauth())
}

/// Credential-based flows: account, session, password, email verification.
fn auth() -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", post(service::register))
        .route("/login", post(service::login))
        .route("/logout", post(service::logout))
        .route("/password-forgot", post(service::password_forgot))
        .route("/password-reset", post(service::password_reset))
        .route("/verify-email", post(service::verify_email))
        .route("/verify-email/resend", post(service::resend))
}

/// OAuth flows.
///
/// `/{provider}` and its callback are `GET` because the browser is redirected
/// through them. `/pending` and `/complete` cover the case where the provider
/// account is not linked yet: the front end reads the pending state, then posts
/// the missing details — and `/complete` issues a session, so it goes through
/// the same path as [`service::login`].
fn oauth() -> Router<Arc<AppState>> {
    Router::new()
        .route("/pending", get(service::pending))
        .route("/complete", post(service::complete))
        .route("/{provider}", get(service::oauth))
        .route("/{provider}/callback", get(service::callback))
}
