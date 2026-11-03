use std::sync::Arc;

use domain::value_objects::Email;
use validations_rs::{
    AsyncBusinessLogicValidation, AsyncValidationError, BusinessRuleError, RuleCode, ValidationFuture,
};

use crate::ports::UserRepository;

/// Business rule that needs I/O: the email is not registered yet.
pub struct EmailNotTaken {
    repo: Arc<dyn UserRepository>,
}

impl EmailNotTaken {
    pub const CODE: RuleCode = RuleCode("EMAIL_TAKEN");

    pub fn new(repo: Arc<dyn UserRepository>) -> Self {
        Self { repo }
    }
}

impl AsyncBusinessLogicValidation<Email> for EmailNotTaken {
    fn validate<'a>(&'a self, email: &'a Email) -> ValidationFuture<'a> {
        Box::pin(async move {
            // A storage failure is not a broken rule: report it as `CheckFailed`.
            let taken = self.repo.exists_by_email(email).await.map_err(AsyncValidationError::check_failed)?;
            if taken {
                Err(BusinessRuleError::new(Self::CODE, "the email is already registered")
                    .with_detail("email", email.as_str())
                    .into())
            } else {
                Ok(())
            }
        })
    }
}
