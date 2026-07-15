use std::sync::Arc;

use super::{
    client::{Client, create_client},
    config::ServiceUrl,
    middleware::cors::cors,
    modules::admin::admin_routes,
    modules::auth::auth_routes,
    modules::contact::contact_routes,
    modules::feed::feed_routes,
    modules::me::me_routes,
    modules::polls::polls_routes,
    modules::rooms::rooms_routes,
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
        .merge(admin_routes())
        .merge(auth_routes())
        .merge(contact_routes())
        .merge(feed_routes())
        .merge(me_routes())
        .merge(polls_routes())
        .merge(rooms_routes())
        .layer(cors_layer)
        .with_state(app_state)
        .layer(TraceLayer::new_for_http())
}
