//! Session records in Redis.
//!
//! Two key families, kept in sync:
//!
//! ```text
//! session:{session_id}       hash, field `userId`  — resolve a cookie
//! user_sessions:{user_id}    hash, field per sid   — list a user's sessions
//! ```
//!
//! The reverse index exists so that logging out everywhere — account deletion,
//! password change — does not require scanning the keyspace. Both carry the
//! same TTL and are expired, refreshed and deleted together.

use redis::{AsyncCommands, RedisResult, aio::MultiplexedConnection};
use tracing::{debug, warn};

use crate::modules::redis::service::{RedisLookup, is_infra_error};

/// Looks up the user behind a session id.
///
/// The hot path: called on every request carrying a `sid` cookie. A missing key
/// is [`RedisLookup::NotFound`] (expired or revoked session), a Redis failure is
/// [`RedisLookup::Infra`] — see [`is_infra_error`].
pub async fn get_session_user(
    con: &mut MultiplexedConnection,
    session_id: &str,
) -> RedisLookup<String> {
    let key = session_key(session_id);
    let value: RedisResult<String> = con.hget(&key, "userId").await;

    match value {
        Ok(user_id) => RedisLookup::Found(user_id),
        Err(error) if is_infra_error(&error) => RedisLookup::Infra(error),
        Err(_) => RedisLookup::NotFound,
    }
}

/// Slides the session expiry, but only in the second half of its life.
///
/// Refreshing on every request would mean two extra round trips per call for a
/// TTL that barely moved. Waiting until the remaining time drops to half the
/// window keeps sessions alive for active users while leaving the common case
/// at a single read.
///
/// The reverse index field is refreshed with the session key so the two never
/// drift apart. Failures are logged, never propagated: a session that keeps its
/// old TTL is a minor annoyance, a rejected request is not.
///
/// Redis TTL sentinels are surfaced as warnings because both mean the data is
/// inconsistent: `-2` is a key that vanished between the read and this call,
/// `-1` a session that was written without an expiry.
pub async fn refresh_ttl(
    con: &mut MultiplexedConnection,
    session_id: &str,
    user_id: &str,
    ttl: i64,
) {
    let key = session_key(session_id);

    match con.ttl(&key).await {
        Ok(-2) => warn!("the key: {key} does not exist"),
        Ok(-1) => warn!("the key: {key} exists but has no associated expire"),
        Ok(current_ttl) if current_ttl <= ttl / 2 => {
            let result: RedisResult<()> = con.expire(&key, ttl).await;
            if let Err(error) = result {
                warn!(error = %error, "failed to refresh session ttl");
            }

            let key = user_sessions(user_id);
            let result: RedisResult<Vec<i32>> = con
                .hexpire(key, ttl, redis::ExpireOption::NONE, &[session_id])
                .await;
            match result {
                Ok(codes) => match codes.as_slice() {
                    // One code per field asked for: 1 means the TTL was set.
                    [1] => {}
                    [-2] => warn!("the field: {session_id} does not exist in {user_id}"),
                    _ => warn!(codes = ?codes, "unexpected hexpire response"),
                },
                Err(error) => warn!(error = %error, "failed to refresh user_sessions ttl"),
            }
            debug!(user_id = %user_id ,session_id = %session_id, "ttl refresh");
        }
        // Still in the first half of its life: nothing to do.
        Ok(_) => {}
        Err(error) => {
            warn!(error = %error, "redis unavailable, passing through without touching expire")
        }
    }
}

/// Revokes one session and removes it from the user's index.
///
/// Used when a session is replaced (login while already logged in) or ended
/// (logout). Errors are logged only — the caller has already decided the
/// session is gone, and a stale Redis entry expires on its own.
pub async fn invalidate_session(con: &mut MultiplexedConnection, session_id: &str, user_id: &str) {
    let session_key = session_key(session_id);
    let user_key = user_sessions(user_id);

    let result: RedisResult<()> = con.del(&session_key).await;
    if let Err(error) = result {
        warn!(error = %error, "failed to delete old session");
    }

    let result: RedisResult<i64> = con.hdel(&user_key, session_id).await;
    if let Err(error) = result {
        warn!(error = %error, "failed to delete old session from user_session");
    }
}

/// Revokes every session a user holds, on every device.
///
/// Reads the ids from the reverse index, deletes all session keys in one call,
/// then drops the index itself. Called on account deletion, where leaving a
/// live session behind would keep a deleted account usable.
pub async fn invalidate_all_sessions(con: &mut MultiplexedConnection, user_id: &str) {
    let user_key = user_sessions(user_id);

    let session_ids: RedisResult<Vec<String>> = con.hkeys(&user_key).await;
    let session_ids = match session_ids {
        Ok(ids) => ids,
        Err(error) => {
            warn!(error = %error, "failed to list sessions for user");
            return;
        }
    };

    // `DEL` with an empty key list is an error, so stop here.
    if session_ids.is_empty() {
        return;
    }

    let session_keys: Vec<String> = session_ids.iter().map(|id| session_key(id)).collect();

    let result: RedisResult<()> = con.del(&session_keys).await;
    if let Err(error) = result {
        warn!(error = %error, "failed to delete sessions for user");
    }

    let result: RedisResult<()> = con.del(&user_key).await;
    if let Err(error) = result {
        warn!(error = %error, "failed to delete user_sessions for user");
    }
}

/// Key holding one session: `session:{session_id}`.
fn session_key(session_id: &str) -> String {
    format!("session:{session_id}")
}

/// Key holding a user's session index: `user_sessions:{user_id}`.
fn user_sessions(user_id: &str) -> String {
    format!("user_sessions:{user_id}")
}
