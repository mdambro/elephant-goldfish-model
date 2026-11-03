use application::ports::{RepoFuture, UserRepository};
use deadpool_postgres::Pool;
use domain::entities::User;
use domain::value_objects::{Email, UserId};

use super::storage_error::storage_error;

/// `UserRepository` backed by PostgreSQL through a connection pool.
pub struct PgUserRepository {
    pool: Pool,
}

impl PgUserRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

impl UserRepository for PgUserRepository {
    fn exists_by_email<'a>(&'a self, email: &'a Email) -> RepoFuture<'a, bool> {
        Box::pin(async move {
            let client = self.pool.get().await.map_err(storage_error)?;
            let row = client
                .query_one("SELECT EXISTS (SELECT 1 FROM users WHERE email = $1)", &[&email.as_str()])
                .await
                .map_err(storage_error)?;
            let exists: bool = row.get(0);
            Ok(exists)
        })
    }

    fn next_id(&self) -> RepoFuture<'_, UserId> {
        Box::pin(async move {
            let client = self.pool.get().await.map_err(storage_error)?;
            let row = client.query_one("SELECT nextval('users_id_seq')", &[]).await.map_err(storage_error)?;
            let id: i64 = row.get(0);
            u64::try_from(id).map(UserId).map_err(storage_error)
        })
    }

    fn save<'a>(&'a self, user: &'a User) -> RepoFuture<'a, ()> {
        Box::pin(async move {
            let client = self.pool.get().await.map_err(storage_error)?;
            let id = i64::try_from(user.id.0).map_err(storage_error)?;
            client
                .execute("INSERT INTO users (id, email) VALUES ($1, $2)", &[&id, &user.email.as_str()])
                .await
                .map_err(storage_error)?;
            Ok(())
        })
    }
}
