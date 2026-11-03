/// Raw input of the "register user" use case, as the adapter received it.
#[derive(Debug, Clone)]
pub struct RegisterUserCommand {
    pub email: String,
    pub age: u8,
    pub accepted_terms: bool,
}
