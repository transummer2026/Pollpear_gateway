use crate::app::AppState;
use crate::modules::apikey::service::{get_user_by_api_key, hash_api_key};
use crate::modules::redis::service::RedisLookup;
use axum::{
    extract::{Request, State},
    http::HeaderValue,
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use tracing::{debug, warn};

pub async fn apikey(
    State(appstate): State<Arc<AppState>>,
    mut request: Request,
    next: Next,
) -> Response {
    let api_key: Option<String> = request
        .headers()
        .get("x-api-key")
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned);

    let Some(api_key) = api_key else {
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
