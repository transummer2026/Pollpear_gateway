//! Handlers for other users' profiles and relationships.
//!
//! Every one is a plain forward to the users service. Authorisation lives
//! upstream, which compares the `x-user-id` the gateway injected against the
//! `{id}` in the path — the gateway itself never decides who may act on whom.

use std::sync::Arc;

use axum::{
    extract::{Request, State},
    response::Response,
};
use hyper::StatusCode;

use crate::{app::AppState, proxy::proxy_forward};

/// `GET /api/users/{id}` — a user's public profile.
pub async fn get_one(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.users_service_url.clone(),
        req,
    )
    .await
}

/// `PATCH /api/users/{id}` — updates a user.
///
/// Moderation endpoint: for their own profile a user goes through
/// `PATCH /api/me/`.
pub async fn update(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.users_service_url.clone(),
        req,
    )
    .await
}

/// `DELETE /api/users/{id}` — deletes a user (moderation).
pub async fn remove(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.users_service_url.clone(),
        req,
    )
    .await
}

/// `POST /api/users/{id}/report` — reports a user.
pub async fn report(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.users_service_url.clone(),
        req,
    )
    .await
}

/// `GET /api/users/{id}/block` — whether the caller blocks this user.
pub async fn get_block_status(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.users_service_url.clone(),
        req,
    )
    .await
}

/// `GET /api/users/{id}/blocks` — the users this user blocks.
pub async fn get_blocked_users(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.users_service_url.clone(),
        req,
    )
    .await
}

/// `POST /api/users/{id}/block` — blocks a user.
pub async fn block(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.users_service_url.clone(),
        req,
    )
    .await
}

/// `DELETE /api/users/{id}/block` — unblocks a user.
pub async fn unblock(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.users_service_url.clone(),
        req,
    )
    .await
}

/// `GET /api/users/{id}/friendship` — the friendship state with this user.
///
/// One resource for the whole relationship, which is why request, acceptance
/// and removal below are all methods on the same path.
pub async fn get_friendship(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.users_service_url.clone(),
        req,
    )
    .await
}

/// `GET /api/users/{id}/friends` — this user's friends.
pub async fn get_friends(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.users_service_url.clone(),
        req,
    )
    .await
}

/// `GET /api/users/{id}/friend-requests` — pending requests for this user.
pub async fn get_friend_requests(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.users_service_url.clone(),
        req,
    )
    .await
}

/// `POST /api/users/{id}/friendship` — sends a friend request.
pub async fn create_friend_request(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.users_service_url.clone(),
        req,
    )
    .await
}

/// `PATCH /api/users/{id}/friendship` — accepts or declines a request.
pub async fn update_friend_request(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.users_service_url.clone(),
        req,
    )
    .await
}

/// `DELETE /api/users/{id}/friendship` — removes a friendship or cancels a request.
pub async fn remove_friendship(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.users_service_url.clone(),
        req,
    )
    .await
}

/// `POST /api/users/{id}/ban` — bans a user (moderation).
pub async fn ban(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.users_service_url.clone(),
        req,
    )
    .await
}

/// `PATCH /api/users/{id}/ban` — amends a ban, for instance its duration.
pub async fn update_ban(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.users_service_url.clone(),
        req,
    )
    .await
}

/// `DELETE /api/users/{id}/ban` — lifts a ban.
pub async fn remove_ban(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.users_service_url.clone(),
        req,
    )
    .await
}
