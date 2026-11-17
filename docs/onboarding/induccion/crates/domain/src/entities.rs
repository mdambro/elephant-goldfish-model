//! Entities: values with an identity that lasts over time.

use crate::value_objects::{Email, UserId};

/// A registered user.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: UserId,
    pub email: Email,
}
