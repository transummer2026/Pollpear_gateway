//! Pollpear API gateway.
//!
//! Single public entry point in front of the Pollpear back end. Every browser
//! and third-party request lands here first; the gateway then:
//!
//! 1. reads its whole configuration from Vault at boot ([`modules::vault`]),
//! 2. authenticates the caller — session cookie for the web app, API key for
//!    `/api/v1` — and turns that into a trusted `x-user-id` header,
//! 3. forwards the request to the matching upstream service ([`proxy`]),
//!    including WebSocket upgrades and S3/Garage media reads.
//!
//! Upstream services never see the `sid` cookie nor a client-supplied
//! `x-user-id`: identity is resolved here and injected here. See [`app`] for
//! the middleware ordering that guarantees it.

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

/// TCP port the gateway listens on. Fixed: the deployment maps it externally.
const PORT: &str = "8080";

/// Boots the gateway: tracing, Vault bootstrap, router assembly, then serve.
///
/// Fails fast — if Vault or Redis is unreachable the process exits rather than
/// starting a gateway that could not authenticate anyone.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    // Only the Vault coordinates come from the environment; everything else
    // (service URLs, TTLs, quotas) is read from Vault below.
    let config = Config::from_env();

    let (service_url, settings) =
        bootstrap(&config.vault_url, config.vault_role, config.vault_secret).await?;

    let app = app_routes(service_url, settings).await?;

    run(app, PORT.to_string()).await?;

    Ok(())
}
