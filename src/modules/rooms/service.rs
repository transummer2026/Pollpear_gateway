//! Handlers for rooms, their invitations, participants and messages.
//!
//! All plain forwards to the rooms service, which checks membership against the
//! injected `x-user-id`. Live message delivery is not here: it goes over the
//! socket relayed by [`crate::modules::ws`], and these endpoints cover history
//! and administration.

use std::sync::Arc;

use axum::{
    extract::{Request, State},
    response::Response,
};
use hyper::StatusCode;

use crate::{app::AppState, proxy::proxy_forward};

/// `POST /api/rooms/new` — creates a room.
pub async fn create(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.rooms_service_url.clone(),
        req,
    )
    .await
}

/// `GET /api/rooms/` — the caller's rooms.
pub async fn read(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.rooms_service_url.clone(),
        req,
    )
    .await
}

/// `GET /api/rooms/{id}` — one room.
pub async fn read_one(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.rooms_service_url.clone(),
        req,
    )
    .await
}

/// `GET /api/rooms/feed` — rooms shown on the feed page.
///
/// Declared after `/{id}` but matched before it: axum prefers the literal
/// segment over the parameter, so `/rooms/feed` never reaches [`read_one`].
pub async fn read_many(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.rooms_service_url.clone(),
        req,
    )
    .await
}

/// `PATCH /api/rooms/{id}` — updates a room.
pub async fn update(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.rooms_service_url.clone(),
        req,
    )
    .await
}

/// `DELETE /api/rooms/{id}` — deletes a room.
pub async fn remove(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.rooms_service_url.clone(),
        req,
    )
    .await
}

/// `GET /api/rooms/{id}/invitations` — invitations sent for this room.
///
/// The room side of the relationship; the invitee's own list is
/// `GET /api/me/invitations`.
pub async fn get_invitations(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.rooms_service_url.clone(),
        req,
    )
    .await
}

/// `GET /api/rooms/{id}/invitations/{userId}` — one invitation.
pub async fn get_invitation(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.rooms_service_url.clone(),
        req,
    )
    .await
}

/// `POST /api/rooms/{id}/invitations/{userId}` — invites a user.
pub async fn create_invitation(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.rooms_service_url.clone(),
        req,
    )
    .await
}

/// `PATCH /api/rooms/{id}/invitations/{userId}` — accepts or declines an invitation.
pub async fn respond_invitation(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.rooms_service_url.clone(),
        req,
    )
    .await
}

/// `DELETE /api/rooms/{id}/invitations/{userId}` — withdraws an invitation.
pub async fn remove_invitation(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.rooms_service_url.clone(),
        req,
    )
    .await
}

/// `POST /api/rooms/{id}/participants` — adds a participant.
///
/// The direct path, distinct from the invitation flow above: the user id is in
/// the body rather than the URL, since no invitation resource is involved.
pub async fn add_participant(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.rooms_service_url.clone(),
        req,
    )
    .await
}

/// `DELETE /api/rooms/{id}/participants/{userId}` — removes a participant, or leaves the room.
pub async fn remove_participant(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.rooms_service_url.clone(),
        req,
    )
    .await
}

/// `GET /api/rooms/{id}/messages` — message history.
pub async fn get_messages(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.rooms_service_url.clone(),
        req,
    )
    .await
}

/// `POST /api/rooms/{id}/messages` — posts a message.
///
/// Persistence path. Fan-out to connected clients happens over the WebSocket
/// session, which the gateway only relays.
pub async fn create_message(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.rooms_service_url.clone(),
        req,
    )
    .await
}
