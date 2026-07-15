use std::sync::Arc;

use axum::{
    extract::{Request, State},
    response::Response,
};
use hyper::StatusCode;

use crate::{app::AppState, proxy::proxy_forward};

pub async fn my_profile(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client.clone(),
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
        appstate.client.clone(),
        appstate.service_url.me_service_url.clone(),
        req,
    )
    .await
}

pub async fn remove(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client.clone(),
        appstate.service_url.me_service_url.clone(),
        req,
    )
    .await
}

pub async fn upload_avatar(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client.clone(),
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
        appstate.client.clone(),
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
        appstate.client.clone(),
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
        appstate.client.clone(),
        appstate.service_url.me_service_url.clone(),
        req,
    )
    .await
}
