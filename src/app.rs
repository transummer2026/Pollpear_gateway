use std::{error::Error, sync::Arc};

use crate::{
    clients::connect_redis,
    middleware::{
        session::session,
        strip::{sid::sid, x_user_id::x_user_id},
    },
};

use super::{
    clients::{ClientHTTP, create_client_http},
    config::ServiceUrl,
    config::Settings,
    middleware::cors::cors,
    modules::admin::admin_routes,
    modules::api::api_routes,
    modules::auth::auth_routes,
    modules::contact::contact_routes,
    modules::feed::feed_routes,
    modules::healthcheck::healthcheck_routes,
    modules::me::me_routes,
    modules::polls::polls_routes,
    modules::rooms::rooms_routes,
    modules::search::search_routes,
    modules::uploads::uploads_routes,
    modules::users::users_routes,
    modules::ws::ws_routes,
};
use axum::{Router, extract::FromRef, middleware};
use redis::aio::MultiplexedConnection;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[derive(Clone)]
pub struct AppState {
    pub client_http: ClientHTTP,
    pub con_redis: MultiplexedConnection,
    pub service_url: ServiceUrl,
    pub settings: Settings,
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

pub async fn app_routes(service_url: ServiceUrl, settings: Settings) -> Result<Router, Box<dyn Error>> {
    let client_http = create_client_http();
    let con_redis = connect_redis(&service_url.redis_url).await?;
    let cors_layer = cors(&service_url.frontend_url);
    let app_state = Arc::new(AppState {
        client_http,
        con_redis,
        service_url,
        settings,
    });
    Ok(Router::new()
        .nest(
            "/api",
            Router::new()
                .merge(public_routes())
                .merge(protected_routes(&app_state))
                .merge(api_routes(&app_state)),
        )
        .merge(healthcheck_routes())
        .route_layer(middleware::from_fn(x_user_id))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(cors_layer),
        )
        .with_state(app_state))
}

fn public_routes() -> Router<Arc<AppState>> {
    Router::new()
        .merge(auth_routes())
        .merge(contact_routes())
        .merge(uploads_routes())
}

fn protected_routes(app_state: &Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .merge(admin_routes())
        .merge(feed_routes())
        .merge(me_routes())
        .merge(polls_routes())
        .merge(rooms_routes())
        .merge(search_routes())
        .merge(users_routes())
        .merge(ws_routes())
        .route_layer(middleware::from_fn(sid))
        .route_layer(middleware::from_fn_with_state(app_state.clone(), session))
}
