use crate::errors::DomainError;

/// An email address that passed the format check. The only way to build one
/// is `TryFrom<&str>`, so holding an `Email` proves the check ran.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<&str> for Email {
    type Error = DomainError;

    fn try_from(raw: &str) -> Result<Self, Self::Error> {
        let normalized = raw.trim().to_lowercase();
        match normalized.split_once('@') {
            Some((local, domain)) if !local.is_empty() && domain.contains('.') => Ok(Email(normalized)),
            _ => Err(DomainError::InvalidEmail(raw.to_string())),
        }
    }
}
