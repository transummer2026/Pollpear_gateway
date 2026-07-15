use std::error::Error;

use axum::Router;

pub async fn run(app: Router, port: String) -> Result<(), Box<dyn Error>> {
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;

    axum::serve(listener, app).await?;

    Ok(())
}
