//! Router assembly and shared application state.
//!
//! # Route layout
//!
//! ```text
//! /health                      healthcheck, no auth, outside /api
//! /api/auth/**                 public   — session is created here
//! /api/contact                 public
//! /api/uploads/*path           public   — proxied to Garage (S3)
//! /api/{admin,feed,me,polls,rooms,search,users,ws}
//!                              protected — session cookie required
//! /api/v1/**                   API key + rate limit
//! ```
//!
//! # Middleware order
//!
//! Layers added later wrap the ones added before, so on the request path the
//! chain runs outside-in:
//!
//! ```text
//! TraceLayer → CORS → x_user_id → [ session → sid ]      (protected routes)
//!                              → [ ratelimit → apikey ]  (/api/v1)
//! ```
//!
//! That ordering is the gateway's security invariant:
//!
//! - [`x_user_id`] runs first and drops any client-supplied `x-user-id`, so the
//!   header can only ever be set by the gateway itself;
//! - [`session`] then resolves the `sid` cookie against Redis and re-injects a
//!   trusted `x-user-id`;
//! - [`sid`] runs last and strips the `sid` cookie, so upstream services never
//!   receive the session secret.

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

/// State shared by every handler, held behind an [`Arc`].
///
/// Cheap to clone: the HTTP client and the Redis connection are handles, and
/// the two config structs are read-only for the lifetime of the process.
#[derive(Clone)]
pub struct AppState {
    /// Pooled client used to reach upstream services and Garage.
    pub client_http: ClientHTTP,
    /// Redis connection backing sessions, API keys and rate limiting.
    pub con_redis: MultiplexedConnection,
    /// Upstream addresses resolved from Vault at boot.
    pub service_url: ServiceUrl,
    /// Policy values (session TTL, rate limit) resolved from Vault at boot.
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

/// Builds the complete router and the state it carries.
///
/// Connects to Redis on the way — a failure here aborts startup, since no
/// request could be authenticated without it.
///
/// `/health` is merged outside the `/api` nest so probes stay reachable
/// independently of the API surface. The `x_user_id` guard is added with
/// [`Router::route_layer`] so it applies to matched routes only, leaving 404s
/// to short-circuit without running middleware.
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

/// Routes reachable without a session.
///
/// `auth` is where a session is created, `contact` accepts anonymous messages,
/// and `uploads` serves public media straight from Garage.
fn public_routes() -> Router<Arc<AppState>> {
    Router::new()
        .merge(auth_routes())
        .merge(contact_routes())
        .merge(uploads_routes())
}

/// Routes behind the session cookie.
///
/// [`session`] is added last so it wraps [`sid`]: the cookie is read and
/// resolved to an `x-user-id` first, then stripped before the request leaves
/// the gateway.
///
/// Note that [`session`] does not reject unauthenticated callers — it only
/// resolves identity when a valid cookie is present. Enforcement lives
/// upstream, which sees either a trusted `x-user-id` or none at all. The one
/// exception is [`crate::modules::ws`], which rejects the handshake itself
/// because a WebSocket upgrade cannot be replayed.
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
