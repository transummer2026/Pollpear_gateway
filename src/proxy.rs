use super::clients::ClientHTTP;
use axum::{
    extract::{OriginalUri, Request},
    http::uri::Uri,
    response::{IntoResponse, Response},
};
use hyper::StatusCode;
use tracing::{debug, error};

pub async fn proxy_forward(
    client_http: ClientHTTP,
    service_url: String,
    mut req: Request,
) -> Result<Response, StatusCode> {
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
    let target = format!("{service_url}{path_query}");

    debug!(method = %method, path = %path_query, target = %target, "forwarding");

    *req.uri_mut() = Uri::try_from(target).map_err(|_| StatusCode::BAD_GATEWAY)?;

    let response = client_http.request(req).await.map_err(|e| {
        error!(method = %method, path = %path_query, error = %e, "proxy request failed");
        StatusCode::BAD_GATEWAY
    })?;

    let status = response.status();
    debug!(method = %method, path = %path_query, status = %status, "proxy response");

    Ok(response.into_response())
}
