use domain::errors::DomainError;
use validations_rs::{AsyncValidationError, BusinessRuleError};

use crate::ports::RepositoryError;

/// Everything that can go wrong when registering a user.
#[derive(Debug, thiserror::Error)]
pub enum RegisterUserError {
    /// The input breaks a format invariant (domain).
    #[error(transparent)]
    InvalidInput(#[from] DomainError),
    /// The input breaks a business rule (application).
    #[error(transparent)]
    BusinessRule(#[from] BusinessRuleError),
    /// Storage failed while saving.
    #[error(transparent)]
    Repository(#[from] RepositoryError),
    /// A rule could not be checked (for example, the database did not answer).
    #[error("a business rule could not be checked")]
    RuleCheckFailed(#[source] AsyncValidationError),
}

/// A violated async rule is a business error; a failed check is not.
impl From<AsyncValidationError> for RegisterUserError {
    fn from(error: AsyncValidationError) -> Self {
        match error {
            AsyncValidationError::Violation(rule) => RegisterUserError::BusinessRule(rule),
            other => RegisterUserError::RuleCheckFailed(other),
        }
    }
}
