mod app;
mod client;
mod config;
mod middleware;
mod modules;
mod proxy;
mod server;

use std::error::Error;

use crate::{app::app_routes, config::Config, server::run};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let Config { port, service_url } = Config::from_env();

    let app = app_routes(service_url);

    run(app, port).await?;

    Ok(())
}
