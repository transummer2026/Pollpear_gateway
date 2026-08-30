mod app;
mod clients;
mod config;
mod middleware;
mod modules;
mod proxy;
mod server;

use std::error::Error;

use crate::{
    app::app_routes, config::Config, modules::vault::service::bootstrap, server::run,
};

const PORT: &str = "8080";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let config = Config::from_env();

    let (service_url, settings) =
        bootstrap(&config.vault_url, config.vault_role, config.vault_secret).await?;

    let app = app_routes(service_url, settings).await?;

    run(app, PORT.to_string()).await?;

    Ok(())
}
