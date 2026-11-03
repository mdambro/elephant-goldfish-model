use std::sync::Arc;

use domain::entities::User;
use domain::value_objects::Email;
use validations_rs::{start_validations, start_validations_async};

use crate::commands::RegisterUserCommand;
use crate::errors::RegisterUserError;
use crate::ports::UserRepository;
use crate::validations::{AdultOnly, EmailNotTaken, TermsAccepted};

/// One use case, one responsibility: register a user.
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
        // 2. Cheap business rules first, in order.
        start_validations(&[&AdultOnly, &TermsAccepted], &command)?;
        // 3. Rules that need I/O, only if the cheap ones passed.
        start_validations_async(&[&self.email_not_taken], &email).await?;

        let user = User { id: self.repo.next_id().await?, email };
        self.repo.save(&user).await?;
        Ok(user)
    }
}
