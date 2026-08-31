use axum::{
    extract::{Request, State},
    response::{IntoResponse, Response},
};
use axum_extra::extract::CookieJar;
use hyper::StatusCode;
use std::sync::Arc;

use crate::{
    app::AppState, middleware::strip::sid::clear_sid,
    modules::session::service::invalidate_all_sessions, proxy::proxy_forward,
};

pub async fn my_profile(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.me_service_url.clone(),
        req,
    )
    .await
}
pub async fn edit_profile(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.me_service_url.clone(),
        req,
    )
    .await
}

pub async fn remove(
    State(appstate): State<Arc<AppState>>,
    jar: CookieJar,
    req: Request,
) -> Result<Response, StatusCode> {
    let user_id = req
        .headers()
        .get("x-user-id")
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned);

    let response = proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.me_service_url.clone(),
        req,
    )
    .await?;

    if let Some(user_id) = user_id
        && response.status().is_success()
    {
        let mut con_redis = appstate.con_redis.clone();
        invalidate_all_sessions(&mut con_redis, &user_id).await;
        return Ok((clear_sid(jar), response).into_response());
    }

    Ok(response)
}

pub async fn upload_avatar(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.me_service_url.clone(),
        req,
    )
    .await
}
pub async fn my_settings(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.me_service_url.clone(),
        req,
    )
    .await
}

pub async fn edit_settings(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.me_service_url.clone(),
        req,
    )
    .await
}

pub async fn change_password(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.me_service_url.clone(),
        req,
    )
    .await
}

pub async fn create_apikey(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.me_service_url.clone(),
        req,
    )
    .await
}

pub async fn read_apikey(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.me_service_url.clone(),
        req,
    )
    .await
}
pub async fn rename_apikey(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.me_service_url.clone(),
        req,
    )
    .await
}

pub async fn revoke_apikey(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.me_service_url.clone(),
        req,
    )
    .await
}

pub async fn get_notifications(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.me_service_url.clone(),
        req,
    )
    .await
}

pub async fn get_room_invitations(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.me_service_url.clone(),
        req,
    )
    .await
}

pub async fn read_notifications(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.me_service_url.clone(),
        req,
    )
    .await
}

pub async fn dismiss_notification(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.me_service_url.clone(),
        req,
    )
    .await
}
