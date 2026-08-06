use std::sync::Arc;

use crate::app::AppState;

use super::service;
use axum::{Router, routing::get, routing::post};

pub fn auth_routes() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/auth", auth())
        .nest("/auth/oauth", oauth())
}

fn auth() -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", post(service::register))
        .route("/login", post(service::login))
        .route("/logout", post(service::logout))
        .route("/password-forgot", post(service::password_forgot))
        .route("/password-reset", post(service::password_reset))
}

fn oauth() -> Router<Arc<AppState>> {
    Router::new()
        .route("/pending", get(service::pending))
        .route("/complete", post(service::complete))
        .route("/{provider}", get(service::oauth))
        .route("/{provider}/callback", get(service::callback))
}
