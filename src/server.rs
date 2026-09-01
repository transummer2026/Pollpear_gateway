//! HTTP listener.

use std::error::Error;

use axum::Router;

/// Binds `0.0.0.0:{port}` and serves `app` until the process is stopped.
///
/// Bound on all interfaces because the gateway runs in a container and is
/// reached through the deployment's port mapping.
pub async fn run(app: Router, port: String) -> Result<(), Box<dyn Error>> {
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;

    axum::serve(listener, app).await?;

    Ok(())
}
