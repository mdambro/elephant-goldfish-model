use axum::routing::post;
use axum::Router;

use super::register_user_handler::register_user;
use super::AppState;

/// Routes of the REST API.
pub fn router(state: AppState) -> Router {
    Router::new().route("/users", post(register_user)).with_state(state)
}
