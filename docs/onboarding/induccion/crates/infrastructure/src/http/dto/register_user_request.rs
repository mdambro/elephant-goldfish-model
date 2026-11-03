use application::commands::RegisterUserCommand;
use serde::Deserialize;

/// JSON body of `POST /users`.
#[derive(Debug, Deserialize)]
pub struct RegisterUserRequest {
    pub email: String,
    pub age: u8,
    pub accepted_terms: bool,
}

impl From<RegisterUserRequest> for RegisterUserCommand {
    fn from(request: RegisterUserRequest) -> Self {
        RegisterUserCommand { email: request.email, age: request.age, accepted_terms: request.accepted_terms }
    }
}
