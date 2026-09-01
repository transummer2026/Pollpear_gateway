//! Drops any client-supplied `x-user-id`.

use axum::{extract::Request, middleware::Next, response::Response};

/// Removes `x-user-id` from the incoming request.
///
/// Runs before every other middleware (see [`crate::app::app_routes`]), which
/// is what makes the header trustworthy: after this point only the gateway can
/// set it, so an upstream service can treat its presence as proof of an
/// authenticated caller. Without it, anyone could impersonate any user by
/// sending the header themselves.
pub async fn x_user_id(mut request: Request, next: Next) -> Response {
    request.headers_mut().remove("x-user-id");
    next.run(request).await
}
