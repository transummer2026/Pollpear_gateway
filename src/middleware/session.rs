use crate::app::AppState;
use axum::{
    extract::{Request, State},
    http::HeaderValue,
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum_extra::extract::CookieJar;
use redis::{AsyncCommands, RedisResult};
use std::sync::Arc;
use tracing::debug;

pub async fn session(
    State(appstate): State<Arc<AppState>>,
    jar: CookieJar,
    mut request: Request,
    next: Next,
) -> Response {
    let sid: Option<String> = jar.get("sid").map(|cookie| cookie.value().to_owned());

    let Some(session_id) = sid else {
        debug!("no session cookie");
        return next.run(request).await;
    };

    let mut con_redis = appstate.con_redis.clone();
    let key = format!("session:{}", session_id);
    let value: RedisResult<String> = con_redis.hget(&key, "userId").await;

    match value {
        Ok(user_id) => {
            if let Ok(header_value) = HeaderValue::from_str(&user_id) {
                request.headers_mut().insert("x-user-id", header_value);
                debug!(user_id = %user_id, "session resolved");
            }
            next.run(request).await
        }
        Err(_) => {
            debug!("session not found in redis, invalidating cookie");
            let response = next.run(request).await;
            (jar.remove("sid"), response).into_response()
        }
    }
}
