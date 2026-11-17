//! Business rules shared by several use cases. Rules used by one use case live in its module.

use std::sync::Arc;

use domain::value_objects::Email;
use validations_rs::{AsyncError, AsyncRule, RuleCode, RuleFuture, Violation};

use crate::ports::UserRepository;

/// The email is already registered.
pub const EMAIL_TAKEN: RuleCode = RuleCode("EMAIL_TAKEN");

/// The email is not registered yet. A type, not a function, because it holds a dependency (the
/// repository), and async because checking it needs I/O.
pub struct EmailNotTaken {
    repo: Arc<dyn UserRepository>,
}

impl EmailNotTaken {
    pub fn new(repo: Arc<dyn UserRepository>) -> Self {
        Self { repo }
    }
}

impl AsyncRule<Email> for EmailNotTaken {
    fn check<'a>(&'a self, email: &'a Email) -> RuleFuture<'a> {
        Box::pin(async move {
            // A storage failure is not a broken rule: report it as `CheckFailed`.
            let taken = self.repo.exists_by_email(email).await.map_err(AsyncError::check_failed)?;
            if taken {
                return Err(Violation::new(EMAIL_TAKEN, "the email is already registered")
                    .at_field("email")
                    .with_detail("email", email.as_str())
                    .into());
            }
            Ok(())
        })
    }
}
