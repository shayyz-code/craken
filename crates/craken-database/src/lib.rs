pub use async_trait;
use craken_http::error::CrakenError;
pub use craken_macros::Model;
use sqlx::{postgres::PgPool, sqlite::SqlitePool};

pub mod migration;
pub mod model;
pub mod repository;

/// Enum for different database connection pools.
#[derive(Clone)]
pub enum DatabaseConnection {
    Postgres(PgPool),
    Sqlite(SqlitePool),
}

impl DatabaseConnection {
    pub async fn connect(database_url: &str) -> anyhow::Result<Self> {
        if database_url.starts_with("postgres") {
            let pool = PgPool::connect(database_url).await?;
            Ok(DatabaseConnection::Postgres(pool))
        } else {
            let pool = SqlitePool::connect(database_url).await?;
            Ok(DatabaseConnection::Sqlite(pool))
        }
    }
}

/// Database abstraction layer wrapping a connection pool.
#[derive(Clone)]
pub struct Database {
    pool: DatabaseConnection,
}

impl Database {
    pub async fn connect(database_url: &str) -> anyhow::Result<Self> {
        let pool = DatabaseConnection::connect(database_url).await?;
        Ok(Self { pool })
    }

    pub fn pool(&self) -> &DatabaseConnection {
        &self.pool
    }

    /// Helper to map SQLx errors to CrakenError.
    pub fn map_error(err: sqlx::Error) -> CrakenError {
        match err {
            sqlx::Error::RowNotFound => CrakenError::NotFound("record not found".to_string()),
            _ => CrakenError::Internal(err.to_string()),
        }
    }
}

/// Transaction wrapper for scoped database operations.
pub struct Transaction<'a> {
    pub(crate) tx: sqlx::Transaction<'a, sqlx::Postgres>,
}

impl<'a> Transaction<'a> {
    pub async fn commit(self) -> Result<(), sqlx::Error> {
        self.tx.commit().await
    }

    pub async fn rollback(self) -> Result<(), sqlx::Error> {
        self.tx.rollback().await
    }
}
