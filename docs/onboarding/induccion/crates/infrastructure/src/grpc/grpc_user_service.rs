use std::sync::Arc;

use application::commands::RegisterUserCommand;
use application::use_cases::RegisterUserUseCase;
use proto::users::v1::user_service_server::UserService;
use proto::users::v1::{RegisterUserRequest, RegisterUserResponse};
use tonic::{Request, Response, Status};

use super::status_mapping::status_from;

/// gRPC adapter: translates protobuf messages to the use case and back.
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
