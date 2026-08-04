use std::error::Error;

use axum::body::Body;
use hyper_util::{client::legacy::connect::HttpConnector, rt::TokioExecutor};
use redis::aio::MultiplexedConnection;
use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};

pub type ClientHTTP = hyper_util::client::legacy::Client<HttpConnector, Body>;

pub fn create_client_http() -> ClientHTTP {
    hyper_util::client::legacy::Client::<(), ()>::builder(TokioExecutor::new())
        .build(HttpConnector::new())
}

pub async fn connect_redis(redis_url: &str) -> Result<MultiplexedConnection, Box<dyn Error>> {
    let client = redis::Client::open(redis_url)?;
    let con = client.get_multiplexed_async_connection().await?;

    Ok(con)
}

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
