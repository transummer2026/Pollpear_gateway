use std::sync::Arc;

use super::{
    client::{Client, create_client},
    config::ServiceUrl,
    middleware::cors::cors,
    modules::auth::auth_routes,
};
use axum::{Router, extract::FromRef};
use tower_http::trace::TraceLayer;

#[derive(Clone)]
pub struct AppState {
    pub client: Client,
    pub service_url: ServiceUrl,
}

impl FromRef<AppState> for Client {
    fn from_ref(app_state: &AppState) -> Client {
        app_state.client.clone()
    }
}

impl FromRef<AppState> for ServiceUrl {
    fn from_ref(app_state: &AppState) -> ServiceUrl {
        app_state.service_url.clone()
    }
}

pub fn app_routes(service_url: ServiceUrl) -> Router {
    let client = create_client();
    let cors_layer = cors(&service_url.frontend_url);
    let app_state = Arc::new(AppState {
        client,
        service_url,
    });
    Router::new()
        .merge(auth_routes())
        .layer(cors_layer)
        .with_state(app_state)
        .layer(TraceLayer::new_for_http())
}
