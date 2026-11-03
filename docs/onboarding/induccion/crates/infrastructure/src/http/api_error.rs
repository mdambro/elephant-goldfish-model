use application::errors::RegisterUserError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

/// HTTP translation of application errors. Clients decide by `code`, never by `message`.
pub struct ApiError(RegisterUserError);

#[derive(Serialize)]
struct ErrorBody {
    code: String,
    message: String,
}

impl From<RegisterUserError> for ApiError {
    fn from(error: RegisterUserError) -> Self {
        Self(error)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, code, message) = match &self.0 {
            RegisterUserError::InvalidInput(e) => {
                (StatusCode::UNPROCESSABLE_ENTITY, "INVALID_INPUT".to_string(), e.to_string())
            }
            RegisterUserError::BusinessRule(rule) => {
                (StatusCode::UNPROCESSABLE_ENTITY, rule.code.to_string(), rule.message.clone())
            }
            RegisterUserError::RuleCheckFailed(_) => {
                (StatusCode::SERVICE_UNAVAILABLE, "UNAVAILABLE".to_string(), "try again later".to_string())
            }
            RegisterUserError::Repository(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL".to_string(), "internal error".to_string())
            }
        };
        if status.is_server_error() {
            eprintln!("request failed: {}", self.0); // replace with `tracing::error!` in a real service
        }
        (status, Json(ErrorBody { code, message })).into_response()
    }
}
