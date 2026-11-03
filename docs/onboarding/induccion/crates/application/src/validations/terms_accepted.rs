use validations_rs::{AssertTrueValidation, BusinessLogicValidation, BusinessRuleError};

use crate::commands::RegisterUserCommand;

/// Business rule: the terms must be accepted.
pub struct TermsAccepted;

impl BusinessLogicValidation<RegisterUserCommand> for TermsAccepted {
    fn validate(&self, command: &RegisterUserCommand) -> Result<(), BusinessRuleError> {
        AssertTrueValidation::new().with_message("you must accept the terms").validate(&command.accepted_terms)
    }
}
