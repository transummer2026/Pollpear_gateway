//! Outbound clients shared by the whole gateway.
//!
//! Each of these is built once at boot and cloned into every handler: the HTTP
//! client keeps its connection pool, and the Redis multiplexed connection
//! pipelines all commands over a single socket.

use std::error::Error;

use axum::body::Body;
use hyper_util::{client::legacy::connect::HttpConnector, rt::TokioExecutor};
use redis::aio::MultiplexedConnection;
use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};

/// Pooled HTTP client used for every upstream call.
///
/// Bodies are [`axum::body::Body`] so an incoming request can be forwarded
/// as-is — no buffering, streaming preserved in both directions.
pub type ClientHTTP = hyper_util::client::legacy::Client<HttpConnector, Body>;

/// Builds the upstream HTTP client.
///
/// Plain [`HttpConnector`], no TLS: upstreams are reached over the internal
/// network, and public TLS is terminated in front of the gateway.
pub fn create_client_http() -> ClientHTTP {
    hyper_util::client::legacy::Client::<(), ()>::builder(TokioExecutor::new())
        .build(HttpConnector::new())
}

/// Opens the shared Redis connection (sessions, API keys, rate limiting).
///
/// Multiplexed: a single connection cloned across tasks, commands pipelined.
/// Failing here aborts startup — without Redis nothing can be authenticated.
pub async fn connect_redis(redis_url: &str) -> Result<MultiplexedConnection, Box<dyn Error>> {
    let client = redis::Client::open(redis_url)?;
    let con = client.get_multiplexed_async_connection().await?;

    Ok(con)
}

/// Builds an unauthenticated Vault client.
///
/// The token is deliberately empty: it is obtained through the AppRole login in
/// [`crate::modules::vault::service::approle_login`], which sets it on the
/// client returned here.
pub fn create_client_vault(vault_url: &str) -> Result<VaultClient, Box<dyn Error>> {
    let token: &str = "";

    let con = VaultClient::new(
        VaultClientSettingsBuilder::default()
            .address(vault_url)
            .token(token)
            .build()?,
    )?;

    Ok(con)
}
