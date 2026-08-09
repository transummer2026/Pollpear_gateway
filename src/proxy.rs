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

type BackendSocket = WebSocketStream<MaybeTlsStream<TcpStream>>;

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
