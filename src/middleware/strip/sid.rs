use axum::{extract::Request, http::HeaderValue, middleware::Next, response::Response};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use hyper::header::COOKIE;
use tracing::debug;

pub async fn sid(mut request: Request, next: Next) -> Response {
    remove_sid(&mut request);
    next.run(request).await
}

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

pub fn extract_sid(header_value: &str) -> Option<&str> {
    header_value
        .split("; ")
        .find_map(|part| part.strip_prefix("sid="))
}

pub fn clear_sid(jar: CookieJar) -> CookieJar {
    jar.remove(Cookie::build("sid").path("/").build())
}
