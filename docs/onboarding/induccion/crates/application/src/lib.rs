//! Application layer: use cases, the ports they need, and the business rules they share.
//!
//! Each use case is a module with everything that changes with it: its command, its error, its
//! private rules, and the use case struct. Code shared by several use cases lives in `ports` and
//! `rules`.

pub mod ports;
pub mod register_user;
pub mod rules;
