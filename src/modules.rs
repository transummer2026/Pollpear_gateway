//! Feature modules.
//!
//! Two kinds live side by side:
//!
//! - **Routed modules** — [`admin`], [`api`], [`auth`], [`contact`], [`feed`],
//!   [`healthcheck`], [`me`], [`polls`], [`rooms`], [`search`], [`uploads`],
//!   [`users`], [`ws`]. Each has a `controller` declaring the routes and a
//!   `service` holding the handlers, and each exposes a single `*_routes()`
//!   function that [`crate::app`] merges.
//! - **Infrastructure modules** — [`apikey`], [`ratelimit`], [`redis`],
//!   [`session`], [`vault`]. No routes; they hold the Redis and Vault logic the
//!   middlewares and handlers build on.
//!
//! Most routed handlers are a single [`crate::proxy::proxy_forward`] call. The
//! ones that do more are the ones that touch session state: [`auth`] (login,
//! logout), [`me`] (account deletion), plus [`uploads`] and [`ws`], which
//! forward to Garage and to the backend socket instead of an HTTP service.

pub mod admin;
pub mod api;
pub mod apikey;
pub mod auth;
pub mod contact;
pub mod feed;
pub mod healthcheck;
pub mod me;
pub mod polls;
pub mod ratelimit;
pub mod redis;
pub mod rooms;
pub mod search;
pub mod session;
pub mod uploads;
pub mod users;
pub mod vault;
pub mod ws;
