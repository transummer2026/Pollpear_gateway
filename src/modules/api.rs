//! Public `/api/v1` surface, authenticated by API key.

mod controller;
mod service;

pub use controller::api_routes;
