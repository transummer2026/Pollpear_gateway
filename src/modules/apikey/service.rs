use axum::http::HeaderMap;
use redis::{AsyncCommands, RedisResult, aio::MultiplexedConnection};
use sha2::{Digest, Sha256};

use crate::modules::redis::service::{RedisLookup, is_infra_error};

pub fn extract_api_key(headers: &HeaderMap) -> Option<String> {
    headers
        .get("x-api-key")
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned)
}

pub async fn get_user_by_api_key(
    con: &mut MultiplexedConnection,
    key_hash: &str,
) -> RedisLookup<String> {
    let key = api_key_key(key_hash);
    let value: RedisResult<String> = con.get(&key).await;

    match value {
        Ok(user_id) => RedisLookup::Found(user_id),
        Err(error) if is_infra_error(&error) => RedisLookup::Infra(error),
        Err(_) => RedisLookup::NotFound,
    }
}

pub fn hash_api_key(api_key: &str) -> String {
    let digest = Sha256::digest(api_key.as_bytes());
    hex::encode(digest)
}

fn api_key_key(key_hash: &str) -> String {
    format!("apikey:{key_hash}")
}
