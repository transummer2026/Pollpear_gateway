//! Request middlewares.
//!
//! Two families:
//!
//! - **identity and quota** — [`session`] (cookie → `x-user-id`), [`apikey`]
//!   (`x-api-key` → `x-user-id`) and [`ratelimit`] (per-key quota);
//! - **stripping** — [`strip`], which removes the headers upstream must never
//!   see, whether they came from the client or from the gateway.
//!
//! [`cors`] is the odd one out: a plain tower layer, applied globally.
//!
//! All of them fail *open* on a Redis outage — the request goes through
//! unauthenticated rather than the gateway returning 500 for everyone. Upstream
//! still sees no `x-user-id`, so an outage degrades callers to anonymous
//! instead of granting anything.

pub mod apikey;
pub mod cors;
pub mod ratelimit;
pub mod session;
pub mod strip;
