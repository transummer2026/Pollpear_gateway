//! Handlers for the caller's own account.
//!
//! All of them forward to the me service, which identifies the subject from the
//! `x-user-id` the session middleware injected. [`remove`] is the exception: it
//! also has to tear down the sessions the deleted account still holds.

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

/// `GET /api/me/` — the caller's profile.
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
/// `PATCH /api/me/` — updates the caller's profile.
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

/// `DELETE /api/me/` — deletes the account and logs it out everywhere.
///
/// The upstream deletes the user, but the gateway owns the sessions: without
/// this, every device still logged in would keep a working `x-user-id` for a
/// user that no longer exists, until each session's TTL ran out.
///
/// So the `x-user-id` is captured before forwarding — it is the only place the
/// identity appears — and on success every session is revoked
/// ([`invalidate_all_sessions`]) and the caller's own cookie cleared. On
/// failure the response passes through untouched and the sessions stay valid.
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

/// `POST /api/me/avatar` — uploads a new avatar, body streamed through.
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

/// `PUT /api/me/settings/security/` — changes the password.
///
/// Session revocation is left to the auth service, which decides whether a
/// password change should log other devices out.
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

/// `POST /api/me/settings/security/apikey` — issues an API key.
///
/// Upstream generates the key, stores its hash in Redis where
/// [`crate::middleware::apikey::apikey`] will find it, and returns the plain
/// value once — the gateway only ever handles hashes afterwards.
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

/// `GET /api/me/settings/security/apikey` — lists the caller's keys (metadata only).
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
/// `PATCH /api/me/settings/security/apikey` — renames a key.
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

/// `DELETE /api/me/settings/security/apikey` — revokes a key.
///
/// Upstream drops the Redis entry, so the key stops resolving on the next
/// `/api/v1` request.
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

/// `GET /api/me/notifications/` — lists notifications.
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

/// `GET /api/me/invitations` — room invitations received by the caller.
///
/// The invitee's view; the room owner's is
/// [`crate::modules::rooms`]'s `/rooms/{id}/invitations`.
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

/// `PATCH /api/me/notifications/read` — marks notifications as read.
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

/// `DELETE /api/me/notifications/{id}` — dismisses one notification.
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
