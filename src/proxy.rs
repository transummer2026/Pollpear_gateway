//! Forwarding primitives: HTTP pass-through and WebSocket relaying.
//!
//! Everything routed by the gateway ends up here. HTTP requests go through
//! [`proxy_forward`]; WebSocket sessions are split into two independent
//! [`proxy_upstream`] / [`proxy_downstream`] pumps.

use super::clients::ClientHTTP;
use axum::{
    extract::{
        OriginalUri, Request,
        ws::{Message as FrontMsg, WebSocket},
    },
    http::uri::Uri,
    response::{IntoResponse, Response},
};
use futures_util::{
    SinkExt, StreamExt,
    stream::{SplitSink, SplitStream},
};
use hyper::StatusCode;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, tungstenite::Message as BackMsg};
use tracing::{debug, error};

/// Client-side socket to the backend, TLS-capable.
type BackendSocket = WebSocketStream<MaybeTlsStream<TcpStream>>;

/// Returns the URI the client actually sent.
///
/// [`Router::nest`](axum::Router::nest) rewrites `req.uri()` to the path
/// *relative to the nest*, which would drop the `/api` prefix and any parent
/// segment. Axum stores the untouched URI in the [`OriginalUri`] extension, so
/// forwarding must read it from there and only fall back to `req.uri()` when
/// the request was not nested.
pub fn original_uri(req: &Request) -> Uri {
    match req.extensions().get::<OriginalUri>() {
        Some(orig) => orig.0.clone(),
        None => req.uri().clone(),
    }
}

/// Forwards an HTTP request to `service_url` and returns the upstream response.
///
/// The request is passed through as-is — method, headers and streamed body —
/// with only the URI rewritten: the `/api` prefix is stripped, so
/// `GET /api/rooms/42` reaches the upstream as `GET /rooms/42`.
///
/// Headers are *not* filtered here. Stripping happens earlier in the chain
/// ([`crate::middleware::strip`]), which is what keeps the `sid` cookie and any
/// client-supplied `x-user-id` from reaching upstream.
///
/// # Errors
///
/// [`StatusCode::BAD_GATEWAY`] if the rewritten target is not a valid URI or if
/// the upstream call fails. Upstream error statuses are not errors here: they
/// are returned to the client unchanged.
pub async fn proxy_forward(
    client_http: ClientHTTP,
    service_url: String,
    mut req: Request,
) -> Result<Response, StatusCode> {
    let method = req.method().clone();
    let uri_owned = original_uri(&req);
    let path_query = uri_owned
        .path_and_query()
        .map(|v| v.as_str())
        .unwrap_or_else(|| uri_owned.path())
        .to_owned();
    let path_query = path_query.strip_prefix("/api").unwrap_or(&path_query);
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

/// Pumps browser → backend frames until either side closes.
///
/// A `Close` frame is forwarded first and only then breaks the loop, so the
/// backend learns why the client left instead of seeing a dropped socket. The
/// backend sender is closed on exit whatever the reason, which unblocks
/// [`proxy_downstream`] on the other half of the session.
pub async fn proxy_upstream(
    mut receiver_front: SplitStream<WebSocket>,
    mut sender_back: SplitSink<BackendSocket, BackMsg>,
) {
    while let Some(message) = receiver_front.next().await {
        let message = match message {
            Ok(message) => message,
            Err(error) => {
                error!(error = %error, "front websocket error");
                break;
            }
        };

        let is_close = matches!(message, FrontMsg::Close(_));

        if let Err(error) = sender_back.send(front_to_back(message)).await {
            error!(error = %error, "failed to forward message upstream");
            break;
        }

        if is_close {
            debug!("close received from front, propagating upstream");
            break;
        }
    }

    debug!("closing backend sender");
    let _ = sender_back.close().await;
}

/// Pumps backend → browser frames until either side closes.
///
/// Mirror of [`proxy_upstream`], with one asymmetry: raw
/// [`BackMsg::Frame`] values have no axum equivalent and are skipped rather
/// than relayed.
pub async fn proxy_downstream(
    mut receiver_back: SplitStream<BackendSocket>,
    mut sender_front: SplitSink<WebSocket, FrontMsg>,
) {
    while let Some(message) = receiver_back.next().await {
        let message = match message {
            Ok(message) => message,
            Err(error) => {
                error!(error = %error, "backend websocket error");
                break;
            }
        };

        let is_close = matches!(message, BackMsg::Close(_));

        let Some(message) = back_to_front(message) else {
            continue;
        };

        if let Err(error) = sender_front.send(message).await {
            error!(error = %error, "failed to forward message downstream");
            break;
        }

        if is_close {
            debug!("close received from backend, propagating downstream");
            break;
        }
    }

    debug!("closing front sender");
    let _ = sender_front.close().await;
}

/// Converts an axum frame into its tungstenite equivalent.
///
/// Purely mechanical: the two crates model the same protocol with different
/// types. Payloads are re-borrowed rather than copied where the representation
/// allows it.
fn front_to_back(message: FrontMsg) -> BackMsg {
    match message {
        FrontMsg::Text(t) => BackMsg::Text(t.as_str().into()),
        FrontMsg::Binary(b) => BackMsg::Binary(b),
        FrontMsg::Ping(p) => BackMsg::Ping(p),
        FrontMsg::Pong(p) => BackMsg::Pong(p),
        FrontMsg::Close(c) => {
            BackMsg::Close(
                c.map(|cf| tokio_tungstenite::tungstenite::protocol::CloseFrame {
                    code: cf.code.into(),
                    reason: cf.reason.as_str().into(),
                }),
            )
        }
    }
}

/// Converts a tungstenite frame into its axum equivalent.
///
/// Returns [`None`] for [`BackMsg::Frame`], the raw variant tungstenite exposes
/// for low-level use: axum has no counterpart, and it never appears on a normal
/// message stream, so the caller simply skips it.
fn back_to_front(message: BackMsg) -> Option<FrontMsg> {
    match message {
        BackMsg::Text(t) => Some(FrontMsg::Text(t.as_str().into())),
        BackMsg::Binary(b) => Some(FrontMsg::Binary(b)),
        BackMsg::Ping(p) => Some(FrontMsg::Ping(p)),
        BackMsg::Pong(p) => Some(FrontMsg::Pong(p)),
        BackMsg::Close(c) => Some(FrontMsg::Close(c.map(|cf| axum::extract::ws::CloseFrame {
            code: cf.code.into(),
            reason: cf.reason.as_str().into(),
        }))),
        BackMsg::Frame(_) => None,
    }
}
