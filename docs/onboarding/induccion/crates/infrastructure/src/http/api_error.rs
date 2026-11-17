//! HTTP translation of application errors. Clients decide by `code`, never by `message`.

use application::register_user::RegisterUserError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

pub(super) struct ApiError(RegisterUserError);

#[derive(Serialize)]
struct ErrorBody {
    code: String,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    field: Option<String>,
}

impl ErrorBody {
    fn new(code: &str, message: impl Into<String>) -> Self {
        Self { code: code.to_string(), message: message.into(), field: None }
    }
}

impl From<RegisterUserError> for ApiError {
    fn from(error: RegisterUserError) -> Self {
        Self(error)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, body) = match &self.0 {
            RegisterUserError::InvalidInput(e) => {
                (StatusCode::UNPROCESSABLE_ENTITY, ErrorBody::new("INVALID_INPUT", e.to_string()))
            }
            RegisterUserError::BusinessRule(violation) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                ErrorBody {
                    code: violation.code().to_string(),
                    message: violation.message().to_string(),
                    field: violation.field().map(str::to_string),
                },
            ),
            RegisterUserError::RuleCheckFailed(_) => {
                (StatusCode::SERVICE_UNAVAILABLE, ErrorBody::new("UNAVAILABLE", "try again later"))
            }
            RegisterUserError::Repository(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, ErrorBody::new("INTERNAL", "internal error"))
            }
        };
        if status.is_server_error() {
            eprintln!("request failed: {}", self.0); // replace with `tracing::error!` in a real service
        }
        (status, Json(body)).into_response()
    }
}
