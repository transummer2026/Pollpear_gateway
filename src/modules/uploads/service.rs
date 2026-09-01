//! Garage (S3) media handler.

use std::sync::Arc;

use axum::{
    extract::{OriginalUri, Request, State},
    http::{HeaderValue, uri::Uri},
    response::{IntoResponse, Response},
};
use hyper::{StatusCode, header::HOST};
use tracing::{debug, error};

use crate::app::AppState;

/// `GET /api/uploads/{*path}` — serves an object from Garage.
///
/// Does not use [`crate::proxy::proxy_forward`], because Garage needs two things
/// that generic forwarding does not do:
///
/// - both `/api` *and* `/uploads` are stripped, so `/api/uploads/avatars/x.png`
///   becomes `/avatars/x.png` on the bucket;
/// - the `Host` header is rewritten to `garage_uploads_host`. Garage resolves
///   the bucket from the host name (virtual-hosted addressing), so forwarding
///   the gateway's own `Host` would miss the bucket entirely.
///
/// The response streams straight back to the client — no buffering, so large
/// media does not sit in gateway memory.
///
/// # Errors
///
/// [`StatusCode::BAD_GATEWAY`] if the rewritten target is invalid or Garage is
/// unreachable, [`StatusCode::INTERNAL_SERVER_ERROR`] if the configured uploads
/// host is not a usable header value.
pub async fn upload(
    State(appstate): State<Arc<AppState>>,
    mut req: Request,
) -> Result<Response, StatusCode> {
    let garage_url = appstate.service_url.garage_url.clone();
    let garage_uploads_host = appstate.service_url.garage_uploads_host.clone();

    let method = req.method().clone();
    // Same reasoning as `crate::proxy::original_uri`: nesting rewrote
    // `req.uri()`, so the prefixes to strip are only in the original URI.
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
