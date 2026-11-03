use validations_rs::{BusinessLogicValidation, BusinessRuleError, MinValidation};

use crate::commands::RegisterUserCommand;

/// Business rule: only adults can register.
pub struct AdultOnly;

impl BusinessLogicValidation<RegisterUserCommand> for AdultOnly {
    fn validate(&self, command: &RegisterUserCommand) -> Result<(), BusinessRuleError> {
        MinValidation::new(18u8).with_message("you must be of legal age").validate(&command.age)
    }
}
