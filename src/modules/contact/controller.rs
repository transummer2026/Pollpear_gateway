//! `/api/contact` route.

use std::sync::Arc;

use crate::app::AppState;

use super::service;
use axum::{Router, routing::post};

/// Declares `POST /contact`.
///
/// Public: visitors must be able to reach support without an account.
pub fn contact_routes() -> Router<Arc<AppState>> {
    Router::new().route("/contact", post(service::contact))
}
