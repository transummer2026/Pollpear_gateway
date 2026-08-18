use std::sync::Arc;

use axum::{
    extract::{Request, State},
    response::Response,
};
use hyper::StatusCode;

use crate::{app::AppState, proxy::proxy_forward};

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
