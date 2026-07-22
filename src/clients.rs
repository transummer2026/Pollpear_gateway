use axum::body::Body;
use hyper_util::{client::legacy::connect::HttpConnector, rt::TokioExecutor};
use redis::aio::MultiplexedConnection;
use tracing::error;

pub type ClientHTTP = hyper_util::client::legacy::Client<HttpConnector, Body>;

pub fn create_client_http() -> ClientHTTP {
    hyper_util::client::legacy::Client::<(), ()>::builder(TokioExecutor::new())
        .build(HttpConnector::new())
}

pub async fn connect_redis(redis_url: &str) -> MultiplexedConnection {
    let client = match redis::Client::open(redis_url) {
        Ok(client) => client,
        Err(e) => {
            error!("{e}");
            std::process::exit(1);
        }
    };

    match client.get_multiplexed_async_connection().await {
        Ok(con) => con,
        Err(e) => {
            error!("{e}");
            std::process::exit(1);
        }
    }
}
