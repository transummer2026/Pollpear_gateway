//! Authentication handlers.
//!
//! Most are a plain forward to the auth service. The three that are not —
//! [`login`], [`logout`] and [`complete`] — are the ones where a session begins
//! or ends, and they keep the browser cookie and the Redis record in step:
//! upstream owns credentials and issues the `sid`, the gateway owns the Redis
//! session index and cleans it up.

use std::sync::Arc;
use tracing::{debug, warn};

use axum::{
    extract::{Request, State},
    http::HeaderValue,
    response::{IntoResponse, Response},
};
use axum_extra::extract::CookieJar;
use hyper::{StatusCode, header::SET_COOKIE};

use crate::{
    app::AppState,
    middleware::strip::sid::{clear_sid, extract_sid, remove_sid},
    modules::{
        redis::service::RedisLookup,
        session::service::{get_session_user, invalidate_session},
    },
    proxy::proxy_forward,
};

/// `POST /api/auth/register` — forwards the sign-up request.
///
/// No session work: registering does not log the user in.
pub async fn register(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.auth_service_url.clone(),
        req,
    )
    .await
}
/// `POST /api/auth/login` — logs in, replacing any session already held.
///
/// Logging in while already logged in would otherwise leave the previous
/// session alive in Redis: the browser drops the old cookie the moment upstream
/// sends a new one, so nothing would ever revoke the old record and it would
/// sit there until its TTL ran out — still usable by anyone holding the value.
///
/// Hence the sequence:
///
/// 1. read the incoming `sid` from the jar, then strip it from the forwarded
///    request — upstream authenticates credentials here, not a cookie;
/// 2. resolve which user that old session belonged to, *before* it is replaced;
/// 3. forward, and look for a `sid` in the response's `Set-Cookie` headers;
/// 4. revoke the old session only if all three are present — an old session, a
///    known owner, and a new cookie proving the login succeeded.
///
/// A failed login therefore leaves the existing session untouched.
pub async fn login(
    State(appstate): State<Arc<AppState>>,
    jar: CookieJar,
    mut req: Request,
) -> Result<Response, StatusCode> {
    let req_session_id: Option<String> = jar.get("sid").map(|cookie| cookie.value().to_owned());
    remove_sid(&mut req);

    let mut con_redis = appstate.con_redis.clone();

    // Resolved before forwarding: `invalidate_session` needs the owner to
    // update the reverse index, and after the response the old record may
    // already be unreachable.
    let user_id = match &req_session_id {
        Some(req_session_id) => match get_session_user(&mut con_redis, req_session_id).await {
            RedisLookup::Found(user_id) => Some(user_id),
            RedisLookup::NotFound => {
                debug!("session not found in redis, skipping invalidation of old session");
                None
            }
            RedisLookup::Infra(error) => {
                warn!(error = %error, "redis unavailable, skipping invalidation of old session");
                None
            }
        },
        None => {
            debug!("no session cookie on login request");
            None
        }
    };

    let response = proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.auth_service_url.clone(),
        req,
    )
    .await?;

    // A `sid` in `Set-Cookie` is the signal that upstream accepted the
    // credentials and issued a fresh session.
    let res_session_id = response
        .headers()
        .get_all(SET_COOKIE)
        .iter()
        .find_map(|value| value.to_str().ok().and_then(extract_sid));

    if let (Some(req_session_id), Some(user_id), Some(_)) =
        (&req_session_id, &user_id, &res_session_id)
    {
        invalidate_session(&mut con_redis, req_session_id, user_id).await;
    }

    Ok(response)
}
/// `POST /api/auth/logout` — revokes the session and clears the cookie.
///
/// Unlike the other protected flows this one rejects on its own: without a
/// resolvable session there is nothing to log out of, so it answers `401` and
/// clears the cookie anyway, which unsticks a browser holding a dead `sid`.
///
/// The session is revoked *before* forwarding, on purpose: an upstream failure
/// must not leave a live session behind after the user asked to leave. The
/// forward still happens so the auth service can run its own cleanup, and it
/// gets an explicit `x-user-id` — the cookie has been stripped, so the header
/// is the only identity left on the request.
pub async fn logout(
    State(appstate): State<Arc<AppState>>,
    jar: CookieJar,
    mut req: Request,
) -> Result<Response, StatusCode> {
    let req_session_id: Option<String> = jar.get("sid").map(|cookie| cookie.value().to_owned());
    remove_sid(&mut req);

    let mut con_redis = appstate.con_redis.clone();

    let user_id = match &req_session_id {
        Some(req_session_id) => match get_session_user(&mut con_redis, req_session_id).await {
            RedisLookup::Found(user_id) => Some(user_id),
            RedisLookup::NotFound => {
                debug!("session not found in redis, rejecting logout");
                return Ok((clear_sid(jar), StatusCode::UNAUTHORIZED).into_response());
            }
            // Redis is down: the session may well be valid, so forward the
            // logout and let upstream do what it can. The cookie is cleared
            // below either way, so the browser stops using it.
            RedisLookup::Infra(error) => {
                warn!(error = %error, "redis unavailable, forwarding logout without invalidating session");
                None
            }
        },
        None => {
            debug!("no session cookie on logout request");
            return Ok((clear_sid(jar), StatusCode::UNAUTHORIZED).into_response());
        }
    };

    if let Some(user_id) = &user_id
        && let Ok(header_value) = HeaderValue::from_str(user_id)
    {
        req.headers_mut().insert("x-user-id", header_value);
        debug!(user_id = %user_id, "session resolved");
    }

    if let (Some(req_session_id), Some(user_id)) = (&req_session_id, &user_id) {
        invalidate_session(&mut con_redis, req_session_id, user_id).await;
    }

    let response = proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.auth_service_url.clone(),
        req,
    )
    .await?;

    Ok((clear_sid(jar), response).into_response())
}
/// `POST /api/auth/password-forgot` — forwards the reset-link request.
pub async fn password_forgot(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.auth_service_url.clone(),
        req,
    )
    .await
}
/// `POST /api/auth/password-reset` — forwards the new password and its token.
///
/// Existing sessions are not revoked here; the auth service owns that decision,
/// and it can do it through the same Redis index the gateway uses.
pub async fn password_reset(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.auth_service_url.clone(),
        req,
    )
    .await
}

/// `POST /api/auth/verify-email` — forwards the email verification token.
pub async fn verify_email(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.auth_service_url.clone(),
        req,
    )
    .await
}

/// `POST /api/auth/verify-email/resend` — forwards a request for a new
/// verification email.
pub async fn resend(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.auth_service_url.clone(),
        req,
    )
    .await
}
/// `GET /api/auth/oauth/{provider}` — starts an OAuth flow.
///
/// Forwarded as-is: upstream builds the provider URL and answers with the
/// redirect, which the gateway passes back untouched.
pub async fn oauth(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.auth_service_url.clone(),
        req,
    )
    .await
}
/// `GET /api/auth/oauth/{provider}/callback` — receives the provider redirect.
///
/// Upstream exchanges the code and, for an already linked account, issues the
/// session cookie itself; the gateway just relays the `Set-Cookie`. An account
/// that still needs linking is sent through [`pending`] and [`complete`].
pub async fn callback(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.auth_service_url.clone(),
        req,
    )
    .await
}
/// `GET /api/auth/oauth/pending` — reads the state of an unfinished OAuth link.
///
/// Lets the front end show what the provider returned and ask for whatever is
/// still missing before the account can be created.
pub async fn pending(
    State(appstate): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, StatusCode> {
    proxy_forward(
        appstate.client_http.clone(),
        appstate.service_url.auth_service_url.clone(),
        req,
    )
    .await
}
/// `POST /api/auth/oauth/complete` — finishes an OAuth link and logs in.
///
/// Delegates to [`login`] rather than forwarding directly: this endpoint also
/// ends with upstream issuing a `sid`, so it needs the same replace-the-old-
/// session handling. Keeping it as a delegation means that logic exists once.
pub async fn complete(
    State(appstate): State<Arc<AppState>>,
    jar: CookieJar,
    req: Request,
) -> Result<Response, StatusCode> {
    login(State(appstate), jar, req).await
}
