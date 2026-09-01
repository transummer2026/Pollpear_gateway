//! Header stripping, in both directions of trust.
//!
//! [`x_user_id`](x_user_id::x_user_id) removes what the *client* may have sent;
//! [`sid`](sid::sid) removes what the *gateway* uses internally. Together they
//! make `x-user-id` a header only the gateway can set, and keep the session
//! cookie from ever reaching an upstream service.

pub mod sid;
pub mod x_user_id;
