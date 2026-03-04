use crate::model::Model;
use crate::{Database, DatabaseConnection};
use craken_http::error::CrakenError;
use sqlx::{query_as, Postgres, Sqlite};
use std::sync::Arc;

/// Optional Repository<T> abstraction for models.
///
/// Repositories wrap models to provide cleaner service-level access
/// and allow for easier mocking in tests.
pub struct Repository<T: Model> {
    db: Arc<Database>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Model> Repository<T> {
    pub fn new(db: Arc<Database>) -> Self {
        Self {
            db,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Resolve model by primary key.
    pub async fn find(&self, id: i64) -> Result<T, CrakenError> {
        let sql = format!(
            "SELECT * FROM {} WHERE {} = ?",
            T::table_name(),
            T::primary_key()
        );
        match self.db.pool() {
            DatabaseConnection::Postgres(pool) => {
                query_as::<Postgres, T>(&sql.replace("?", "$1"))
                    .bind(id)
                    .fetch_one(pool)
                    .await
                    .map_err(Database::map_error)
            }
            DatabaseConnection::Sqlite(pool) => {
                query_as::<Sqlite, T>(&sql)
                    .bind(id)
                    .fetch_one(pool)
                    .await
                    .map_err(Database::map_error)
            }
        }
    }

    /// Get all records for this model.
    pub async fn all(&self) -> Result<Vec<T>, CrakenError> {
        let sql = format!("SELECT * FROM {}", T::table_name());
        match self.db.pool() {
            DatabaseConnection::Postgres(pool) => {
                query_as::<Postgres, T>(&sql)
                    .fetch_all(pool)
                    .await
                    .map_err(Database::map_error)
            }
            DatabaseConnection::Sqlite(pool) => {
                query_as::<Sqlite, T>(&sql)
                    .fetch_all(pool)
                    .await
                    .map_err(Database::map_error)
            }
        }
    }

    /// Delete record by primary key.
    pub async fn delete(&self, id: i64) -> Result<u64, CrakenError> {
        let sql = format!(
            "DELETE FROM {} WHERE {} = ?",
            T::table_name(),
            T::primary_key()
        );
        match self.db.pool() {
            DatabaseConnection::Postgres(pool) => {
                sqlx::query(&sql.replace("?", "$1"))
                    .bind(id)
                    .execute(pool)
                    .await
                    .map(|res| res.rows_affected())
                    .map_err(Database::map_error)
            }
            DatabaseConnection::Sqlite(pool) => {
                sqlx::query(&sql)
                    .bind(id)
                    .execute(pool)
                    .await
                    .map(|res| res.rows_affected())
                    .map_err(Database::map_error)
            }
        }
    }
}
