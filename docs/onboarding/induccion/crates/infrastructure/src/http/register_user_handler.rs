use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;

use super::api_error::ApiError;
use super::dto::{RegisterUserRequest, UserResponse};
use super::AppState;

/// `POST /users`: translates HTTP to the use case and back. No business logic here.
pub async fn register_user(
    State(state): State<AppState>,
    Json(request): Json<RegisterUserRequest>, // body extractor goes last
) -> Result<(StatusCode, Json<UserResponse>), ApiError> {
    let user = state.register_user.execute(request.into()).await?;
    Ok((StatusCode::CREATED, Json(user.into())))
}
