use redis::{AsyncCommands, RedisResult, aio::MultiplexedConnection};
use tracing::{debug, warn};

use crate::modules::redis::service::{RedisLookup, is_infra_error};

const TTL: i64 = 60 * 30;

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

pub async fn refresh_ttl(con: &mut MultiplexedConnection, session_id: &str, user_id: &str) {
    let key = session_key(session_id);

    match con.ttl(&key).await {
        Ok(-2) => warn!("the key: {key} does not exist"),
        Ok(-1) => warn!("the key: {key} exists but has no associated expire"),
        Ok(ttl) if ttl <= TTL / 2 => {
            let result: RedisResult<()> = con.expire(&key, TTL).await;
            if let Err(error) = result {
                warn!(error = %error, "failed to refresh session ttl");
            }

            let key = user_sessions(user_id);
            let result: RedisResult<Vec<i32>> = con
                .hexpire(key, TTL, redis::ExpireOption::NONE, &[session_id])
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

fn session_key(session_id: &str) -> String {
    format!("session:{session_id}")
}

fn user_sessions(user_id: &str) -> String {
    format!("user_sessions:{user_id}")
}
