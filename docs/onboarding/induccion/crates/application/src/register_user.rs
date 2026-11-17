//! Use case: register a user. Its command, its error, its rules, and the use case itself live here,
//! because they change together.

use std::sync::Arc;

use domain::entities::User;
use domain::errors::DomainError;
use domain::value_objects::Email;
use validations_rs::{ensure, validate, validate_async, AsyncError, RuleCode, Violation};

use crate::ports::{RepositoryError, UserRepository};
use crate::rules::EmailNotTaken;

/// Raw input of the use case, as the adapter received it.
#[derive(Debug, Clone)]
pub struct RegisterUserCommand {
    pub email: String,
    pub age: u8,
    pub accepted_terms: bool,
}

/// Everything that can go wrong when registering a user.
#[derive(Debug, thiserror::Error)]
pub enum RegisterUserError {
    /// The input breaks a format invariant (domain).
    #[error(transparent)]
    InvalidInput(#[from] DomainError),
    /// The input breaks a business rule (application).
    #[error(transparent)]
    BusinessRule(#[from] Violation),
    /// Storage failed while saving.
    #[error(transparent)]
    Repository(#[from] RepositoryError),
    /// A rule could not be checked (for example, the database did not answer).
    #[error("a business rule could not be checked")]
    RuleCheckFailed(#[source] AsyncError),
}

/// A broken async rule is a business error; a failed check is not.
impl From<AsyncError> for RegisterUserError {
    fn from(error: AsyncError) -> Self {
        match error {
            AsyncError::Violation(violation) => Self::BusinessRule(violation),
            other => Self::RuleCheckFailed(other),
        }
    }
}

/// Only adults can register.
pub const UNDERAGE: RuleCode = RuleCode("UNDERAGE");
/// The terms must be accepted.
pub const TERMS_NOT_ACCEPTED: RuleCode = RuleCode("TERMS_NOT_ACCEPTED");

// Rules of this use case only: private functions, not types. A function is enough for a rule
// with no configuration and no dependencies.
fn adult(command: &RegisterUserCommand) -> Result<(), Violation> {
    ensure(command.age >= 18, UNDERAGE, "you must be of legal age").map_err(|v| v.at_field("age"))
}

fn terms_accepted(command: &RegisterUserCommand) -> Result<(), Violation> {
    ensure(command.accepted_terms, TERMS_NOT_ACCEPTED, "you must accept the terms")
        .map_err(|v| v.at_field("accepted_terms"))
}

/// The use case is a struct because it holds dependencies; its one public operation is `execute`.
pub struct RegisterUserUseCase {
    repo: Arc<dyn UserRepository>,
    email_not_taken: EmailNotTaken,
}

impl RegisterUserUseCase {
    pub fn new(repo: Arc<dyn UserRepository>) -> Self {
        Self { email_not_taken: EmailNotTaken::new(Arc::clone(&repo)), repo }
    }

    pub async fn execute(&self, command: RegisterUserCommand) -> Result<User, RegisterUserError> {
        // 1. Format invariant: owned by the domain newtype.
        let email = Email::try_from(command.email.as_str())?;
        // 2. Cheap business rules first, in order; stop at the first violation.
        validate(&command, &[&adult, &terms_accepted])?;
        // 3. Rules that need I/O, only if the cheap ones passed.
        validate_async(&email, &[&self.email_not_taken]).await?;

        let user = User { id: self.repo.next_id().await?, email };
        self.repo.save(&user).await?;
        Ok(user)
    }
}
