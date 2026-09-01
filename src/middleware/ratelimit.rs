//! Per-API-key quota on `/api/v1`.

use crate::app::AppState;
use crate::modules::apikey::service::{extract_api_key, hash_api_key};
use crate::modules::ratelimit::service::{RateLimitDecision, check_rate_limit};
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
};
use hyper::StatusCode;
use std::sync::Arc;
use tracing::{debug, warn};

/// Counts `/api/v1` requests per API key and rejects those over quota.
///
/// Outermost layer on `/api/v1`, so it runs before
/// [`crate::middleware::apikey::apikey`] resolves the key: counting on the key
/// *hash* means an unknown or revoked key is throttled just like a valid one,
/// which is what keeps the key space from being enumerated.
///
/// Requests with no `x-api-key` are not counted — they will be rejected
/// upstream for lack of identity anyway. Quota and window come from the Vault
/// [`Settings`](crate::config::Settings).
///
/// A Redis outage lets the request through: a broken counter should not take
/// the whole API down.
pub async fn ratelimit(
    State(appstate): State<Arc<AppState>>,
    request: Request,
    next: Next,
) -> Response {
    let Some(api_key) = extract_api_key(request.headers()) else {
        debug!("no api key header");
        return next.run(request).await;
    };

    let key_hash = hash_api_key(&api_key);
    let mut con_redis = appstate.con_redis.clone();

    match check_rate_limit(
        &mut con_redis,
        &key_hash,
        appstate.settings.rate_limit,
        appstate.settings.rate_limit_window_seconds,
    )
    .await
    {
        RateLimitDecision::Allowed => next.run(request).await,
        RateLimitDecision::Exceeded => {
            debug!("rate limit exceeded");
            StatusCode::TOO_MANY_REQUESTS.into_response()
        }
        RateLimitDecision::Infra(error) => {
            warn!(error = %error, "redis unavailable, passing through without rate limiting");
            next.run(request).await
        }
    }
}
