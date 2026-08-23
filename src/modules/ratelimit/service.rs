use redis::{AsyncCommands, RedisError, RedisResult, aio::MultiplexedConnection};

pub enum RateLimitDecision {
    Allowed,
    Exceeded,
    Infra(RedisError),
}

pub async fn check_rate_limit(
    con: &mut MultiplexedConnection,
    key_hash: &str,
    limit: u64,
    window_seconds: i64,
) -> RateLimitDecision {
    let key = rate_limit_key(key_hash);
    let count: RedisResult<u64> = con.incr(&key, 1).await;

    match count {
        Ok(1) => {
            let _: RedisResult<()> = con.expire(&key, window_seconds).await;
            RateLimitDecision::Allowed
        }
        Ok(count) if count <= limit => RateLimitDecision::Allowed,
        Ok(_) => RateLimitDecision::Exceeded,
        Err(error) => RateLimitDecision::Infra(error),
    }
}

fn rate_limit_key(key_hash: &str) -> String {
    format!("ratelimit:{key_hash}")
}
