//! API-key lookups in Redis.
//!
//! Keys live at `apikey:{sha256(key)}` and hold the owner's user id. The
//! gateway only ever sees the hash after [`hash_api_key`], so a dump of the
//! keyspace does not hand out usable credentials.

use axum::http::HeaderMap;
use redis::{AsyncCommands, RedisResult, aio::MultiplexedConnection};
use sha2::{Digest, Sha256};

use crate::modules::redis::service::{RedisLookup, is_infra_error};

/// Reads the `x-api-key` header.
///
/// Returns [`None`] when absent or not valid UTF-8, which the callers treat
/// alike: forward the request unauthenticated.
pub fn extract_api_key(headers: &HeaderMap) -> Option<String> {
    headers
        .get("x-api-key")
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned)
}

/// Looks up the owner of an API key from its hash.
///
/// Takes the hash, never the key itself. [`RedisLookup::NotFound`] means the key
/// is unknown or revoked; [`RedisLookup::Infra`] means Redis could not answer,
/// and the caller fails open.
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

/// Hashes an API key to its lowercase hex SHA-256.
///
/// Plain SHA-256, no salt or work factor, because the input is a
/// high-entropy generated key rather than a password: brute force is
/// infeasible regardless, and the lookup is on the request hot path. The digest
/// must match exactly what the back end stores when it issues a key.
pub fn hash_api_key(api_key: &str) -> String {
    let digest = Sha256::digest(api_key.as_bytes());
    hex::encode(digest)
}

/// Key holding an API key's owner: `apikey:{key_hash}`.
fn api_key_key(key_hash: &str) -> String {
    format!("apikey:{key_hash}")
}
