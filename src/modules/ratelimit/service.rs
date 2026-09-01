//! Fixed-window rate limiting in Redis.

use redis::{AsyncCommands, RedisError, RedisResult, aio::MultiplexedConnection};

/// Outcome of a quota check.
pub enum RateLimitDecision {
    /// Under quota — serve the request.
    Allowed,
    /// Over quota — answer `429`.
    Exceeded,
    /// Redis could not answer; the caller fails open.
    Infra(RedisError),
}

/// Counts one request against `key_hash`'s window and decides on it.
///
/// Fixed window: `INCR` creates the counter at 1 and the same call sets its
/// expiry, so the window starts with the first request of a burst and the key
/// disappears on its own afterwards. Setting the TTL only on that first
/// increment is what keeps the window from sliding forward forever under
/// sustained traffic.
///
/// The trade-off of a fixed window is the boundary: a client can spend its full
/// quota at the end of one window and again at the start of the next. Accepted
/// here — the limit protects the back end from runaway clients, it is not a
/// precise metering device.
///
/// Counting on the key *hash* rather than the resolved user means unknown keys
/// are throttled too; see [`crate::middleware::ratelimit::ratelimit`].
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
            // First request of the window: start the clock. The result is
            // ignored so a failing EXPIRE never costs the caller its request;
            // the cost is a counter left without expiry, which keeps rejecting
            // that key until it is cleared by hand.
            let _: RedisResult<()> = con.expire(&key, window_seconds).await;
            RateLimitDecision::Allowed
        }
        Ok(count) if count <= limit => RateLimitDecision::Allowed,
        Ok(_) => RateLimitDecision::Exceeded,
        Err(error) => RateLimitDecision::Infra(error),
    }
}

/// Key holding a window counter: `ratelimit:{key_hash}`.
fn rate_limit_key(key_hash: &str) -> String {
    format!("ratelimit:{key_hash}")
}
