use axum::{extract::Request, middleware::Next, response::Response};

pub async fn x_user_id(mut request: Request, next: Next) -> Response {
    request.headers_mut().remove("x-user-id");
    next.run(request).await
}
