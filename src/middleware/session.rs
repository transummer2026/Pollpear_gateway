use crate::app::AppState;
use crate::modules::redis::service::RedisLookup;
use crate::modules::session::service::{get_session_user, refresh_ttl};
use axum::{
    extract::{Request, State},
    http::HeaderValue,
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum_extra::extract::cookie::CookieJar;
use std::sync::Arc;
use tracing::{debug, warn};

pub async fn session(
    State(appstate): State<Arc<AppState>>,
    jar: CookieJar,
    mut request: Request,
    next: Next,
) -> Response {
    let session_id: Option<String> = jar.get("sid").map(|cookie| cookie.value().to_owned());

    let Some(session_id) = session_id else {
        debug!("no session cookie");
        return next.run(request).await;
    };

    let mut con_redis = appstate.con_redis.clone();

    match get_session_user(&mut con_redis, &session_id).await {
        RedisLookup::Found(user_id) => {
            if let Ok(header_value) = HeaderValue::from_str(&user_id) {
                request.headers_mut().insert("x-user-id", header_value);
                debug!(user_id = %user_id, "session resolved");
            }
            refresh_ttl(&mut con_redis, &session_id, &user_id).await;
            next.run(request).await
        }
        RedisLookup::NotFound => {
            debug!("session not found in redis, invalidating cookie");
            let response = next.run(request).await;
            (jar.remove("sid"), response).into_response()
        }
        RedisLookup::Infra(error) => {
            warn!(error = %error, "redis unavailable, passing through without touching cookie");
            next.run(request).await
        }
    }
}
