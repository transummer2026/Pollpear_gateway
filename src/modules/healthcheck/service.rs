//! Health handler.

use hyper::StatusCode;
use tracing::debug;

/// Answers `200 OK`.
///
/// Liveness only: it reports that the process is up and serving, and
/// deliberately does not probe Redis, Vault or the back end. Every middleware
/// fails open on a Redis outage, so a gateway that cannot reach Redis is still
/// serving traffic and should not be restarted by the orchestrator.
pub async fn health() -> StatusCode {
    debug!("Health check requested");
    StatusCode::OK
}
