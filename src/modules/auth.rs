//! Authentication: registration, login, logout, password reset, OAuth.
//!
//! The only public module that owns session state. Everything else in the
//! gateway *reads* a session; the handlers here create and destroy them, which
//! is why [`crate::modules::auth::service::login`] and
//! [`crate::modules::auth::service::logout`] do more than forward a request.

mod controller;
mod service;

pub use controller::auth_routes;
