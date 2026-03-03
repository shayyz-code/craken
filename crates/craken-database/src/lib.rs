pub use async_trait;
use craken_http::error::CrakenError;
pub use craken_macros::Model;
use sqlx::postgres::PgPool;

pub mod migration;
pub mod model;
pub mod repository;

/// Database abstraction layer wrapping SQLx PgPool.
pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn connect(url: &str) -> anyhow::Result<Self> {
        let pool = PgPool::connect(url).await?;
        Ok(Self { pool })
    }

    pub fn pool(&self) -> &PgPool {
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
