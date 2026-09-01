//! API-key authentication for the public `/api/v1` surface.

use crate::app::AppState;
use crate::modules::apikey::service::{extract_api_key, get_user_by_api_key, hash_api_key};
use crate::modules::redis::service::RedisLookup;
use axum::{
    extract::{Request, State},
    http::HeaderValue,
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use tracing::{debug, warn};

/// Resolves the `x-api-key` header into a trusted `x-user-id`.
///
/// The `/api/v1` counterpart of [`crate::middleware::session::session`], and it
/// behaves the same way: authentication only. Every outcome — no header, an
/// unknown key, or Redis down — forwards the request without `x-user-id` and
/// lets upstream reject it. Rejecting here would give an oracle telling a
/// caller which keys exist.
///
/// Only the SHA-256 of the key is looked up; see
/// [`hash_api_key`].
///
/// Runs *after* [`crate::middleware::ratelimit::ratelimit`], so unknown keys
/// are throttled too and cannot be brute-forced.
pub async fn apikey(
    State(appstate): State<Arc<AppState>>,
    mut request: Request,
    next: Next,
) -> Response {
    let Some(api_key) = extract_api_key(request.headers()) else {
        debug!("no api key header");
        return next.run(request).await;
    };

    let key_hash = hash_api_key(&api_key);
    let mut con_redis = appstate.con_redis.clone();

    match get_user_by_api_key(&mut con_redis, &key_hash).await {
        RedisLookup::Found(user_id) => {
            if let Ok(header_value) = HeaderValue::from_str(&user_id) {
                request.headers_mut().insert("x-user-id", header_value);
                debug!(user_id = %user_id, "api key resolved");
            }
            next.run(request).await
        }
        RedisLookup::NotFound => {
            debug!("api key not found in redis");
            next.run(request).await
        }
        RedisLookup::Infra(error) => {
            warn!(error = %error, "redis unavailable, passing through without resolving api key");
            next.run(request).await
        }
    }
}
