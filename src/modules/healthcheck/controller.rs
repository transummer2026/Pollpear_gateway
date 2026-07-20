use super::service;
use axum::{Router, routing::get};

pub fn healthcheck_routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new().route("/health", get(service::health))
}
