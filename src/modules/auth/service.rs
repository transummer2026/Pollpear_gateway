use std::sync::Arc;
use tracing::{debug, warn};

use axum::{
    extract::{Request, State},
    http::HeaderValue,
    response::{IntoResponse, Response},
};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use hyper::{StatusCode, header::SET_COOKIE};

use crate::{
    app::AppState,
    middleware::strip::sid::{extract_sid, remove_sid},
    modules::{
        redis::service::RedisLookup,
        session::service::{get_session_user, invalidate_session},
    },
    proxy::proxy_forward,
};

pub async fn register(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.auth_service_url.clone(),
        req,
    )
    .await
}
pub async fn login(
    State(appstate): State<Arc<AppState>>,
    jar: CookieJar,
    mut req: Request,
) -> Result<Response, StatusCode> {
    let req_session_id: Option<String> = jar.get("sid").map(|cookie| cookie.value().to_owned());
    remove_sid(&mut req);

    let mut con_redis = appstate.con_redis.clone();

    let user_id = match &req_session_id {
        Some(req_session_id) => match get_session_user(&mut con_redis, req_session_id).await {
            RedisLookup::Found(user_id) => Some(user_id),
            RedisLookup::NotFound => {
                debug!("session not found in redis, skipping invalidation of old session");
                None
            }
            RedisLookup::Infra(error) => {
                warn!(error = %error, "redis unavailable, skipping invalidation of old session");
                None
            }
        },
        None => {
            debug!("no session cookie on login request");
            None
        }
    };

    let response = proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.auth_service_url.clone(),
        req,
    )
    .await?;

    let res_session_id = response
        .headers()
        .get_all(SET_COOKIE)
        .iter()
        .find_map(|value| value.to_str().ok().and_then(extract_sid));

    if let (Some(req_session_id), Some(user_id), Some(_)) =
        (&req_session_id, &user_id, &res_session_id)
    {
        invalidate_session(&mut con_redis, req_session_id, user_id).await;
    }

    Ok(response)
}
fn clear_sid(jar: CookieJar) -> CookieJar {
    jar.remove(Cookie::build("sid").path("/").build())
}
pub async fn logout(
    State(appstate): State<Arc<AppState>>,
    jar: CookieJar,
    mut req: Request,
) -> Result<Response, StatusCode> {
    let req_session_id: Option<String> = jar.get("sid").map(|cookie| cookie.value().to_owned());
    remove_sid(&mut req);

    let mut con_redis = appstate.con_redis.clone();

    let user_id = match &req_session_id {
        Some(req_session_id) => match get_session_user(&mut con_redis, req_session_id).await {
            RedisLookup::Found(user_id) => Some(user_id),
            RedisLookup::NotFound => {
                debug!("session not found in redis, rejecting logout");
                return Ok((clear_sid(jar), StatusCode::UNAUTHORIZED).into_response());
            }
            RedisLookup::Infra(error) => {
                warn!(error = %error, "redis unavailable, forwarding logout without invalidating session");
                None
            }
        },
        None => {
            debug!("no session cookie on logout request");
            return Ok((clear_sid(jar), StatusCode::UNAUTHORIZED).into_response());
        }
    };

    if let Some(user_id) = &user_id
        && let Ok(header_value) = HeaderValue::from_str(user_id)
    {
        req.headers_mut().insert("x-user-id", header_value);
        debug!(user_id = %user_id, "session resolved");
    }

    if let (Some(req_session_id), Some(user_id)) = (&req_session_id, &user_id) {
        invalidate_session(&mut con_redis, req_session_id, user_id).await;
    }

    let response = proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.auth_service_url.clone(),
        req,
    )
    .await?;

    Ok((clear_sid(jar), response).into_response())
}
pub async fn password_forgot(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.auth_service_url.clone(),
        req,
    )
    .await
}
pub async fn password_reset(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.auth_service_url.clone(),
        req,
    )
    .await
}

pub async fn verify_email(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.auth_service_url.clone(),
        req,
    )
    .await
}

pub async fn resend(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.auth_service_url.clone(),
        req,
    )
    .await
}
pub async fn oauth(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.auth_service_url.clone(),
        req,
    )
    .await
}
pub async fn callback(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.auth_service_url.clone(),
        req,
    )
    .await
}
pub async fn pending(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.auth_service_url.clone(),
        req,
    )
    .await
}
pub async fn complete(
    State(appstate): State<Arc<AppState>>,
    jar: CookieJar,
    req: Request,
) -> Result<Response, StatusCode> {
    login(State(appstate), jar, req).await
}
