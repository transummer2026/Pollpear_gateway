use std::sync::Arc;

use axum::{
    extract::{Request, State},
    response::Response,
};
use hyper::StatusCode;

use crate::{app::AppState, proxy::proxy_forward};

pub async fn feed(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client.clone(),
        appstate.service_url.feed_service_url.clone(),
        req,
    )
    .await
}
