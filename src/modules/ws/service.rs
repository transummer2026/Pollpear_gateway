//! WebSocket handshake and session relaying.
//!
//! The one protected route that rejects unauthenticated callers itself: an
//! HTTP request can be forwarded and refused upstream, but a socket, once
//! upgraded, is long-lived and there is no second chance to check identity.

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

/// `GET /api/ws` — upgrades the connection and relays it to the back end.
///
/// The `x-user-id` read here was put in place by
/// [`crate::middleware::session::session`] and can only come from the gateway,
/// since [`crate::middleware::strip::x_user_id`] dropped any client-supplied
/// one earlier. Its absence therefore means no valid session.
///
/// The query string is captured *before* the upgrade: the handshake request is
/// consumed by [`WebSocketUpgrade::on_upgrade`], so anything needed for the
/// backend connection has to be moved into the closure first.
///
/// # Errors
///
/// [`StatusCode::UNAUTHORIZED`] when no session was resolved. Failures *after*
/// the upgrade cannot be reported as a status — the response has already been
/// sent — so they are logged and close the socket instead.
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

/// Opens the backend socket and wires the two relays together.
///
/// The browser's query string is re-appended to the backend URL so parameters
/// such as a room id survive the hop, and `x-user-id` is set on the handshake
/// request — the backend authenticates the socket from that header alone, since
/// the `sid` cookie never reaches it.
///
/// Both pumps are spawned as independent tasks and this function returns
/// immediately: the session then lives entirely in
/// [`proxy_upstream`] and [`proxy_downstream`], each ending when its own
/// direction closes. Closing either half closes the sink the other is writing
/// to, so one side hanging up tears the whole session down.
///
/// # Errors
///
/// If the target is not a valid client request or the backend refuses the
/// connection. The caller logs it; the browser sees the socket close.
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
