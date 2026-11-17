//! Ports: what the application needs from the outside, as traits. `infrastructure` implements them.

use std::future::Future;
use std::pin::Pin;

use domain::entities::User;
use domain::value_objects::{Email, UserId};

/// A storage failure, without storage-specific types.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("repository error: {0}")]
pub struct RepositoryError(pub String);

/// Boxed future so the trait stays dyn-compatible (`Arc<dyn UserRepository>`).
pub type RepoFuture<'a, T> = Pin<Box<dyn Future<Output = Result<T, RepositoryError>> + Send + 'a>>;

/// Port for the user aggregate. Implemented in `infrastructure` with tokio-postgres.
pub trait UserRepository: Send + Sync {
    fn exists_by_email<'a>(&'a self, email: &'a Email) -> RepoFuture<'a, bool>;
    fn next_id(&self) -> RepoFuture<'_, UserId>;
    fn save<'a>(&'a self, user: &'a User) -> RepoFuture<'a, ()>;
}
