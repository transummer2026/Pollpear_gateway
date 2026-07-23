use std::sync::Arc;

use crate::{
    clients::connect_redis,
    middleware::{session::session, strip::x_user_id::x_user_id},
};

use super::{
    clients::{ClientHTTP, create_client_http},
    config::ServiceUrl,
    middleware::cors::cors,
    modules::admin::admin_routes,
    modules::auth::auth_routes,
    modules::contact::contact_routes,
    modules::feed::feed_routes,
    modules::healthcheck::healthcheck_routes,
    modules::me::me_routes,
    modules::polls::polls_routes,
    modules::rooms::rooms_routes,
    modules::search::search_routes,
    modules::users::users_routes,
};
use axum::{Router, extract::FromRef, middleware};
use redis::aio::MultiplexedConnection;
use tower_http::trace::TraceLayer;

#[derive(Clone)]
pub struct AppState {
    pub client_http: ClientHTTP,
    pub con_redis: MultiplexedConnection,
    pub service_url: ServiceUrl,
}

impl FromRef<AppState> for ClientHTTP {
    fn from_ref(app_state: &AppState) -> ClientHTTP {
        app_state.client_http.clone()
    }
}

impl FromRef<AppState> for MultiplexedConnection {
    fn from_ref(app_state: &AppState) -> MultiplexedConnection {
        app_state.con_redis.clone()
    }
}

impl FromRef<AppState> for ServiceUrl {
    fn from_ref(app_state: &AppState) -> ServiceUrl {
        app_state.service_url.clone()
    }
}

pub async fn app_routes(service_url: ServiceUrl) -> Router {
    let client_http = create_client_http();
    let con_redis = connect_redis(&service_url.redis_url).await;
    let cors_layer = cors(&service_url.frontend_url);
    let app_state = Arc::new(AppState {
        client_http,
        con_redis,
        service_url,
    });
    Router::new()
        .merge(admin_routes())
        .merge(auth_routes())
        .merge(contact_routes())
        .merge(feed_routes())
        .merge(healthcheck_routes())
        .merge(me_routes())
        .merge(polls_routes())
        .merge(rooms_routes())
        .merge(search_routes())
        .merge(users_routes())
        .layer(cors_layer)
        .layer(TraceLayer::new_for_http())
        .route_layer(middleware::from_fn_with_state(app_state.clone(), session))
        .route_layer(middleware::from_fn(x_user_id))
        .with_state(app_state)
}
