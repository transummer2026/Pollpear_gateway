//! CORS policy for the browser front end.

use axum::http::{HeaderValue, header};
use hyper::Method;
use tower_http::cors::{AllowOrigin, CorsLayer};

/// Builds the CORS layer allowing exactly `origin`.
///
/// A single exact origin rather than a wildcard, because credentials are
/// allowed: the session cookie only travels cross-origin if the browser sees a
/// concrete `Access-Control-Allow-Origin`, and `*` is rejected outright when
/// `Access-Control-Allow-Credentials` is set.
///
/// # Panics
///
/// If `origin` — the `frontend_url` Vault secret — is not a valid header value.
/// Deliberate: a malformed origin would silently break every browser request,
/// so it fails at boot instead.
pub fn cors(origin: &str) -> CorsLayer {
    let header_value = origin
        .parse::<HeaderValue>()
        .expect("FRONTEND_URL is not a valid HTTP origin");

    CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_origin(AllowOrigin::exact(header_value))
        // `x-api-key` is absent on purpose: /api/v1 is a server-to-server
        // surface, not called from the browser.
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION, header::ACCEPT])
        .allow_credentials(true)
}
