//! Distinguishing "no such key" from "Redis is down".
//!
//! Every Redis-backed lookup in the gateway needs that distinction: a missing
//! key means the caller is anonymous, an outage means the gateway cannot tell.
//! The two must not be conflated — treating an outage as "not found" would log
//! every user out at once.

use redis::{ErrorKind, RedisError};

/// Outcome of a Redis lookup that may legitimately find nothing.
pub enum RedisLookup<T> {
    /// The key existed; `T` is its value.
    Found(T),
    /// The key does not exist — expected, not an error.
    NotFound,
    /// Redis could not answer. Callers fail open.
    Infra(RedisError),
}

/// Tells whether a [`RedisError`] is an infrastructure failure.
///
/// The redis crate reports a missing key as [`ErrorKind::UnexpectedReturnType`]
/// when the command is typed to a non-optional value: `nil` came back where a
/// [`String`] was expected. That single kind therefore means "absent", and
/// every other kind — connection, timeout, protocol, auth — means the lookup
/// itself failed.
pub fn is_infra_error(error: &RedisError) -> bool {
    error.kind() != ErrorKind::UnexpectedReturnType
}
