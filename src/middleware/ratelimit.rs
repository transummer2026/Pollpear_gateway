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

const RATE_LIMIT: u64 = 100;
const RATE_LIMIT_WINDOW_SECONDS: i64 = 60;

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
        RATE_LIMIT,
        RATE_LIMIT_WINDOW_SECONDS,
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
