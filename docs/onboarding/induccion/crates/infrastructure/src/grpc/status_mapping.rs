use application::errors::RegisterUserError;
use tonic::Status;

/// gRPC translation of application errors. A function, not `From`: both types are foreign
/// to this crate, so the orphan rule forbids `impl From<RegisterUserError> for Status` here.
pub(crate) fn status_from(error: RegisterUserError) -> Status {
    match error {
        RegisterUserError::InvalidInput(e) => Status::invalid_argument(e.to_string()),
        RegisterUserError::BusinessRule(rule) => Status::failed_precondition(rule.to_string()),
        RegisterUserError::RuleCheckFailed(_) => Status::unavailable("a business rule could not be checked"),
        RegisterUserError::Repository(_) => Status::internal("internal error"),
    }
}
