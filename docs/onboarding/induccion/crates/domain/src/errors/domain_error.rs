/// Violations of format invariants of domain values.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DomainError {
    #[error("invalid email: {0}")]
    InvalidEmail(String),
}
