use std::future::Future;
use std::pin::Pin;

use domain::entities::User;
use domain::value_objects::{Email, UserId};

use super::RepositoryError;

/// Boxed future so the trait stays dyn-compatible (`Arc<dyn UserRepository>`).
pub type RepoFuture<'a, T> = Pin<Box<dyn Future<Output = Result<T, RepositoryError>> + Send + 'a>>;

/// Port for the user aggregate. Implemented in `infrastructure` with tokio-postgres.
pub trait UserRepository: Send + Sync {
    fn exists_by_email<'a>(&'a self, email: &'a Email) -> RepoFuture<'a, bool>;
    fn next_id(&self) -> RepoFuture<'_, UserId>;
    fn save<'a>(&'a self, user: &'a User) -> RepoFuture<'a, ()>;
}
