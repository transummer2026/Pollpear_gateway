use std::sync::Arc;

use axum::{
    extract::{OriginalUri, Request, State},
    http::{HeaderValue, uri::Uri},
    response::{IntoResponse, Response},
};
use hyper::{StatusCode, header::HOST};
use tracing::{debug, error};

use crate::app::AppState;

pub async fn upload(
    State(appstate): State<Arc<AppState>>,
    mut req: Request,
) -> Result<Response, StatusCode> {
    let garage_url = appstate.service_url.garage_url.clone();
    let garage_uploads_host = appstate.service_url.garage_uploads_host.clone();

    let method = req.method().clone();
    let uri_owned = match req.extensions().get::<OriginalUri>() {
        Some(orig) => orig.0.clone(),
        None => req.uri().clone(),
    };
    let path_query = uri_owned
        .path_and_query()
        .map(|v| v.as_str())
        .unwrap_or_else(|| uri_owned.path())
        .to_owned();
    let stripped = path_query.strip_prefix("/api").unwrap_or(&path_query);
    let stripped = stripped.strip_prefix("/uploads").unwrap_or(stripped);
    let target = format!("{garage_url}{stripped}");

    debug!(method = %method, path = %path_query, target = %target, "forwarding to garage");

    *req.uri_mut() = Uri::try_from(target).map_err(|_| StatusCode::BAD_GATEWAY)?;

    let host = HeaderValue::from_str(&garage_uploads_host)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    req.headers_mut().insert(HOST, host);

    let response = appstate.client_http.request(req).await.map_err(|e| {
        error!(method = %method, path = %path_query, error = %e, "garage proxy request failed");
        StatusCode::BAD_GATEWAY
    })?;

    let status = response.status();
    debug!(method = %method, path = %path_query, status = %status, "garage proxy response");

    Ok(response.into_response())
}
