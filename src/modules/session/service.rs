use redis::{AsyncCommands, RedisResult, aio::MultiplexedConnection};
use tracing::{debug, warn};

use crate::modules::redis::service::{RedisLookup, is_infra_error};

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
                    [1] => {}
                    [-2] => warn!("the field: {session_id} does not exist in {user_id}"),
                    _ => warn!(codes = ?codes, "unexpected hexpire response"),
                },
                Err(error) => warn!(error = %error, "failed to refresh user_sessions ttl"),
            }
            debug!(user_id = %user_id ,session_id = %session_id, "ttl refresh");
        }
        Ok(_) => {}
        Err(error) => {
            warn!(error = %error, "redis unavailable, passing through without touching expire")
        }
    }
}

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

fn session_key(session_id: &str) -> String {
    format!("session:{session_id}")
}

fn user_sessions(user_id: &str) -> String {
    format!("user_sessions:{user_id}")
}
