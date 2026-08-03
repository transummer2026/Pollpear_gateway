use serde::Deserialize;

pub mod service;

#[derive(Deserialize)]
pub struct App {
    pub frontend_url: String,
    pub backend_url: String,
}

#[derive(Deserialize)]
pub struct Redis {
    pub host: String,
    pub port: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct Session {
    pub ttl: String,
}

#[derive(Deserialize)]
pub struct S3 {
    pub url: String,
    pub uploads_host: String,
}
