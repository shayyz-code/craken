use crate::model::Model;
use crate::Database;
use craken_http::error::CrakenError;
use sqlx::{query_as, Postgres};
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
            "SELECT * FROM {} WHERE {} = $1",
            T::table_name(),
            T::primary_key()
        );
        query_as::<Postgres, T>(&sql)
            .bind(id)
            .fetch_one(self.db.pool())
            .await
            .map_err(Database::map_error)
    }

    /// Get all records for this model.
    pub async fn all(&self) -> Result<Vec<T>, CrakenError> {
        let sql = format!("SELECT * FROM {}", T::table_name());
        query_as::<Postgres, T>(&sql)
            .fetch_all(self.db.pool())
            .await
            .map_err(Database::map_error)
    }

    /// Delete record by primary key.
    pub async fn delete(&self, id: i64) -> Result<u64, CrakenError> {
        let sql = format!(
            "DELETE FROM {} WHERE {} = $1",
            T::table_name(),
            T::primary_key()
        );
        sqlx::query(&sql)
            .bind(id)
            .execute(self.db.pool())
            .await
            .map(|res| res.rows_affected())
            .map_err(Database::map_error)
    }
}
