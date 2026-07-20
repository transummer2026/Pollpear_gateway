use hyper::StatusCode;
use tracing::debug;

pub async fn health() -> StatusCode {
    debug!("Health check requested");
    StatusCode::OK
}
