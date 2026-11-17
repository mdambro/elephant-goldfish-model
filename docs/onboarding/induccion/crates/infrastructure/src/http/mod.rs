//! REST adapter (axum): translates HTTP to the use cases and back. No business logic here.

mod api_error;
mod users;

use std::sync::Arc;

use application::register_user::RegisterUserUseCase;
use axum::routing::post;
use axum::Router;

/// State shared by the HTTP handlers: one field per use case the API exposes.
#[derive(Clone)]
pub struct AppState {
    pub register_user: Arc<RegisterUserUseCase>,
}

/// Routes of the REST API.
pub fn router(state: AppState) -> Router {
    Router::new().route("/users", post(users::register_user)).with_state(state)
}
