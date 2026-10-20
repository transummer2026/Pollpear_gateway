//! Handlers for polls, their options and votes.
//!
//! All plain forwards to the polls service, which enforces ownership and
//! voting rules against the injected `x-user-id`.

use std::sync::Arc;

use axum::{
    extract::{Request, State},
    response::Response,
};
use hyper::StatusCode;

use crate::{app::AppState, proxy::proxy_forward};

/// `POST /api/polls/new` — creates a poll.
pub async fn create(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.polls_service_url.clone(),
        req,
    )
    .await
}

/// `GET /api/polls/` and `GET /api/polls/{id}` — lists polls, or reads one.
///
/// Both routes share this handler: the gateway forwards the path unchanged, so
/// the upstream tells the two apart on its own.
pub async fn read(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.polls_service_url.clone(),
        req,
    )
    .await
}

/// `PATCH /api/polls/{id}` — updates a poll.
pub async fn update(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.polls_service_url.clone(),
        req,
    )
    .await
}

/// `DELETE /api/polls/{id}` — deletes a poll.
pub async fn remove(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.polls_service_url.clone(),
        req,
    )
    .await
}

/// `POST /api/polls/{id}/report` — reports a poll.
pub async fn report(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.polls_service_url.clone(),
        req,
    )
    .await
}

/// `POST /api/polls/{id}/options/{optionId}/report` — reports one option.
pub async fn report_option(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.polls_service_url.clone(),
        req,
    )
    .await
}

/// `DELETE /api/polls/{id}/options/{optionId}` — removes an option.
pub async fn remove_option(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.polls_service_url.clone(),
        req,
    )
    .await
}

/// `POST /api/polls/{id}/vote` — casts the caller's vote.
pub async fn create_vote(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.polls_service_url.clone(),
        req,
    )
    .await
}

/// `PATCH /api/polls/{id}/vote` — changes the caller's vote.
pub async fn update_vote(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.polls_service_url.clone(),
        req,
    )
    .await
}

/// `DELETE /api/polls/{id}/vote` — withdraws the caller's vote.
pub async fn remove_vote(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.polls_service_url.clone(),
        req,
    )
    .await
}

/// `POST /api/polls/{id}/vote/report` — reports vote activity on a poll.
pub async fn report_vote(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.polls_service_url.clone(),
        req,
    )
    .await
}
