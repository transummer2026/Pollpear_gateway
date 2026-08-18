use std::sync::Arc;

use crate::app::AppState;
use crate::middleware::apikey::apikey;

use super::service;
use axum::{Router, middleware, routing::any};

pub fn api_routes(app_state: &Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new().nest("/v1", v1(app_state))
}

fn v1(app_state: &Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/{*path}", any(service::proxy))
        .route_layer(middleware::from_fn_with_state(app_state.clone(), apikey))
}
