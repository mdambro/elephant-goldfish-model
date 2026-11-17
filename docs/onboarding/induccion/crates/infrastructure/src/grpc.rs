//! gRPC adapter (tonic): translates protobuf messages to the use cases and back.

use std::sync::Arc;

use application::register_user::{RegisterUserCommand, RegisterUserError, RegisterUserUseCase};
use proto::users::v1::user_service_server::UserService;
use proto::users::v1::{RegisterUserRequest, RegisterUserResponse};
use tonic::{Request, Response, Status};

pub struct GrpcUserService {
    register_user: Arc<RegisterUserUseCase>,
}

impl GrpcUserService {
    pub fn new(register_user: Arc<RegisterUserUseCase>) -> Self {
        Self { register_user }
    }
}

#[tonic::async_trait]
impl UserService for GrpcUserService {
    async fn register_user(
        &self,
        request: Request<RegisterUserRequest>,
    ) -> Result<Response<RegisterUserResponse>, Status> {
        let message = request.into_inner();
        let age = u8::try_from(message.age).map_err(|_| Status::invalid_argument("age is out of range"))?;
        let command = RegisterUserCommand { email: message.email, age, accepted_terms: message.accepted_terms };

        let user = self.register_user.execute(command).await.map_err(status_from)?;
        Ok(Response::new(RegisterUserResponse { id: user.id.0, email: user.email.as_str().to_string() }))
    }
}

/// gRPC translation of application errors. A function, not `From`: both types are foreign
/// to this crate, so the orphan rule forbids `impl From<RegisterUserError> for Status` here.
fn status_from(error: RegisterUserError) -> Status {
    match error {
        RegisterUserError::InvalidInput(e) => Status::invalid_argument(e.to_string()),
        RegisterUserError::BusinessRule(violation) => Status::failed_precondition(violation.to_string()),
        RegisterUserError::RuleCheckFailed(_) => Status::unavailable("a business rule could not be checked"),
        RegisterUserError::Repository(_) => Status::internal("internal error"),
    }
}
