use std::{error::Error, sync::Arc};

use axum::{
    extract::{Request, State, WebSocketUpgrade, ws::WebSocket},
    http::HeaderValue,
    response::Response,
};

use futures_util::stream::StreamExt;

use tokio_tungstenite::{connect_async, tungstenite::client::IntoClientRequest};
use tracing::{debug, info, warn};

use hyper::StatusCode;

use crate::{
    app::AppState,
    proxy::{original_uri, proxy_downstream, proxy_upstream},
};

pub async fn handshake(
    State(appstate): State<Arc<AppState>>,
    ws: WebSocketUpgrade,
    req: Request,
) -> Result<Response, StatusCode> {
    let Some(user_id) = req
        .headers()
        .get("x-user-id")
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned)
    else {
        warn!("ws handshake rejected: no authenticated session");
        return Err(StatusCode::UNAUTHORIZED);
    };

    debug!(user_id = %user_id, "ws handshake accepted");

    let query = original_uri(&req).query().map(str::to_owned);

    let ws_url = appstate.service_url.ws_url.clone();

    Ok(ws.on_upgrade(move |socket| async move {
        if let Err(error) = handle_socket(socket, ws_url, query, user_id.clone()).await {
            warn!(user_id = %user_id, error = %error, "ws session failed");
        }
    }))
}

pub async fn handle_socket(
    socket: WebSocket,
    ws_url: String,
    query: Option<String>,
    user_id: String,
) -> Result<(), Box<dyn Error>> {
    debug!(user_id = %user_id, ws_url = %ws_url, "connecting to backend websocket");

    let target = match query {
        Some(query) => format!("{ws_url}?{query}"),
        None => ws_url,
    };

    let mut request = target.into_client_request()?;
    if let Ok(header_value) = HeaderValue::from_str(&user_id) {
        request.headers_mut().insert("x-user-id", header_value);
    };

    let (stream, _response) = connect_async(request).await?;

    info!(user_id = %user_id, "ws session started");

    let (sender_back, receiver_back) = stream.split();
    let (sender_front, receiver_front) = socket.split();

    tokio::spawn(proxy_upstream(receiver_front, sender_back));
    tokio::spawn(proxy_downstream(receiver_back, sender_front));

    Ok(())
}
