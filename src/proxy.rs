use super::client::Client;
use axum::{
    extract::{OriginalUri, Request},
    http::uri::Uri,
    response::{IntoResponse, Response},
};
use hyper::StatusCode;

pub async fn proxy_forward(
    client: Client,
    service_url: String,
    mut req: Request,
) -> Result<Response, StatusCode> {
    let original = req.extensions().get::<OriginalUri>().map(|u| u.0.clone());
    let uri_ref = original.as_ref().unwrap_or_else(|| req.uri());

    let path = uri_ref.path();
    let path_query = uri_ref
        .path_and_query()
        .map(|v| v.as_str())
        .unwrap_or(path);

    let uri = format!("{service_url}{path_query}");

    *req.uri_mut() = Uri::try_from(uri).map_err(|_| StatusCode::BAD_GATEWAY)?;

    Ok(client
        .request(req)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .into_response())
}
