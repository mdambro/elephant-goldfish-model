use std::sync::Arc;

use application::use_cases::RegisterUserUseCase;

/// State shared by the HTTP handlers: one field per use case the API exposes.
#[derive(Clone)]
pub struct AppState {
    pub register_user: Arc<RegisterUserUseCase>,
}
