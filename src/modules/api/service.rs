//! `/api/v1` handler.

use std::sync::Arc;

use axum::{
    extract::{Request, State},
    response::Response,
};
use hyper::StatusCode;

use crate::{app::AppState, proxy::proxy_forward};

/// `ANY /api/v1/**` — forwards to the API service.
///
/// Path and method are passed through untouched; the upstream owns routing and
/// authorisation. The gateway's contribution is the `x-user-id` resolved from
/// the API key, or its absence.
pub async fn proxy(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.api_service_url.clone(),
        req,
    )
    .await
}
