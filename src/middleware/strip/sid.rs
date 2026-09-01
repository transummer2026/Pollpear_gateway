//! Session cookie handling: strip it going out, clear it coming back.

use axum::{extract::Request, http::HeaderValue, middleware::Next, response::Response};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use hyper::header::COOKIE;
use tracing::debug;

/// Strips the `sid` cookie before the request leaves the gateway.
///
/// Applied to protected routes, inside [`crate::middleware::session::session`]
/// so the cookie is still available when identity is resolved and gone by the
/// time the request is forwarded.
pub async fn sid(mut request: Request, next: Next) -> Response {
    remove_sid(&mut request);
    next.run(request).await
}

/// Removes the `sid` entry from the `Cookie` header, keeping the others.
///
/// The session identifier is a gateway secret: upstream services authenticate
/// on `x-user-id` and have no use for it, so it is dropped rather than
/// forwarded. Other cookies are preserved, and the header is removed entirely
/// when `sid` was the only one.
///
/// Also called directly by the auth handlers, which need the raw cookie value
/// *and* a request that no longer carries it.
pub fn remove_sid(request: &mut Request) {
    if let Some(cookies) = request.headers().get(COOKIE) {
        debug!(cookies = ?cookies);
        let filtered = cookies
            .to_str()
            .unwrap_or("")
            .split("; ")
            .filter(|part| extract_sid(part).is_none())
            .collect::<Vec<&str>>()
            .join("; ");

        if filtered.is_empty() {
            request.headers_mut().remove(COOKIE);
        } else if let Ok(cookie_filterd) = HeaderValue::from_str(&filtered) {
            request.headers_mut().insert(COOKIE, cookie_filterd);
        }
    };
}

/// Extracts the `sid` value from a cookie string, if present.
///
/// Works on both a `Cookie` request header (several `name=value` pairs) and a
/// single `Set-Cookie` response header, where the session id is the first pair
/// and the attributes follow — which is how the auth handlers detect that
/// upstream just issued a new session.
pub fn extract_sid(header_value: &str) -> Option<&str> {
    header_value
        .split("; ")
        .find_map(|part| part.strip_prefix("sid="))
}

/// Adds a removal `Set-Cookie` for `sid` to the response jar.
///
/// `path("/")` must match the path the cookie was issued with, otherwise the
/// browser keeps a cookie the gateway considers dead. Used wherever a session
/// stops being valid: logout, account deletion, or a cookie pointing at a
/// session Redis no longer knows about.
pub fn clear_sid(jar: CookieJar) -> CookieJar {
    jar.remove(Cookie::build("sid").path("/").build())
}
