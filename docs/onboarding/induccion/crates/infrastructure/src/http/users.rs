//! The `/users` resource: its JSON bodies and its handlers.

use application::register_user::RegisterUserCommand;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use domain::entities::User;
use serde::{Deserialize, Serialize};

use super::api_error::ApiError;
use super::AppState;

/// JSON body of `POST /users`.
#[derive(Debug, Deserialize)]
pub(super) struct RegisterUserRequest {
    email: String,
    age: u8,
    accepted_terms: bool,
}

impl From<RegisterUserRequest> for RegisterUserCommand {
    fn from(request: RegisterUserRequest) -> Self {
        RegisterUserCommand { email: request.email, age: request.age, accepted_terms: request.accepted_terms }
    }
}

/// JSON body returned for a user.
#[derive(Debug, Serialize)]
pub(super) struct UserResponse {
    id: u64,
    email: String,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse { id: user.id.0, email: user.email.as_str().to_string() }
    }
}

/// `POST /users`: registers a user.
pub(super) async fn register_user(
    State(state): State<AppState>,
    Json(request): Json<RegisterUserRequest>, // body extractor goes last
) -> Result<(StatusCode, Json<UserResponse>), ApiError> {
    let user = state.register_user.execute(request.into()).await?;
    Ok((StatusCode::CREATED, Json(user.into())))
}
