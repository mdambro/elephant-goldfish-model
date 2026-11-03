/// A storage failure, without storage-specific types.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("repository error: {0}")]
pub struct RepositoryError(pub String);
