//! Fixed-window rate limiting in Redis.

use redis::{RedisError, RedisResult, aio::MultiplexedConnection};

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
/// Fixed window: `INCR` creates the counter at 1 and `EXPIRE … NX` starts the
/// clock on it, so the window opens with the first request of a burst and the
/// key disappears on its own afterwards.
///
/// The `NX` flag — set an expiry only on a key that has none — is what makes
/// the pair safe to send on *every* request rather than only on the first. It
/// buys two things:
///
/// - the window never slides forward under sustained traffic, since the expiry
///   is left alone once set;
/// - a counter that lost its expiry gets one back on the very next request.
///   Without that, `INCR` creating the key and `EXPIRE` setting its TTL are two
///   commands, and a failure in between leaves a counter that never resets —
///   it only grows, crosses `limit`, and rejects that key permanently until
///   someone deletes it by hand.
///
/// Both commands go out in a single `MULTI`/`EXEC` round trip, so nothing can
/// land between the increment and the expiry, and a failure fails the whole
/// check rather than half of it.
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

    // `EXPIRE` is spelled out rather than taken from the typed helper, which
    // has no room for the `NX` flag.
    let result: RedisResult<(u64, i64)> = redis::pipe()
        .atomic()
        .incr(&key, 1)
        .cmd("EXPIRE")
        .arg(&key)
        .arg(window_seconds)
        .arg("NX")
        .query_async(con)
        .await;

    match result {
        Ok((count, _)) if count <= limit => RateLimitDecision::Allowed,
        Ok(_) => RateLimitDecision::Exceeded,
        Err(error) => RateLimitDecision::Infra(error),
    }
}

/// Key holding a window counter: `ratelimit:{key_hash}`.
fn rate_limit_key(key_hash: &str) -> String {
    format!("ratelimit:{key_hash}")
}
