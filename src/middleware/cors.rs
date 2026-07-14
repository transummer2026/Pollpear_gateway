use axum::http::{HeaderValue, header};
use hyper::Method;
use tower_http::cors::{AllowOrigin, CorsLayer};

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
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION, header::ACCEPT])
        .allow_credentials(true)
}
