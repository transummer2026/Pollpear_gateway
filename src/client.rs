use axum::body::Body;
use hyper_util::{client::legacy::connect::HttpConnector, rt::TokioExecutor};

pub type Client = hyper_util::client::legacy::Client<HttpConnector, Body>;

pub fn create_client() -> Client {
    hyper_util::client::legacy::Client::<(), ()>::builder(TokioExecutor::new())
        .build(HttpConnector::new())
}
