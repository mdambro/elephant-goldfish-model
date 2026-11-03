use domain::entities::User;
use serde::Serialize;

/// JSON body returned for a user.
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: u64,
    pub email: String,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse { id: user.id.0, email: user.email.as_str().to_string() }
    }
}
