use crate::{Database, DatabaseConnection};
use async_trait::async_trait;
use sqlx;

/// Migration trait for Rust-based schema changes.
#[async_trait]
pub trait Migration: Send + Sync {
    /// The unique identifier for this migration (e.g. "202310121200_create_users").
    fn name(&self) -> &'static str;

    /// Apply schema changes.
    async fn up(&self, db: &Database) -> anyhow::Result<()>;

    /// Revert schema changes.
    async fn down(&self, db: &Database) -> anyhow::Result<()>;
}

/// Simple migration runner managing `schema_migrations` table.
pub struct MigrationRunner {
    migrations: Vec<Box<dyn Migration>>,
}

impl MigrationRunner {
    pub fn new() -> Self {
        Self {
            migrations: Vec::new(),
        }
    }

    pub fn add(&mut self, migration: Box<dyn Migration>) {
        self.migrations.push(migration);
    }

    /// Ensure `schema_migrations` table exists.
    async fn ensure_table(&self, db: &Database) -> anyhow::Result<()> {
        match db.pool() {
            DatabaseConnection::Postgres(pool) => {
                sqlx::query("CREATE TABLE IF NOT EXISTS schema_migrations (version VARCHAR(255) PRIMARY KEY)")
                    .execute(pool)
                    .await?;
            }
            DatabaseConnection::Sqlite(pool) => {
                sqlx::query(
                    "CREATE TABLE IF NOT EXISTS schema_migrations (version TEXT PRIMARY KEY)",
                )
                .execute(pool)
                .await?;
            }
        }
        Ok(())
    }

    /// Run all pending migrations.
    pub async fn run_pending(&self, db: &Database) -> anyhow::Result<()> {
        self.ensure_table(db).await?;

        for migration in &self.migrations {
            let version = migration.name();
            let applied: bool = match db.pool() {
                DatabaseConnection::Postgres(pool) => {
                    sqlx::query_scalar(
                        "SELECT EXISTS(SELECT 1 FROM schema_migrations WHERE version = $1)",
                    )
                    .bind(version)
                    .fetch_one(pool)
                    .await?
                }
                DatabaseConnection::Sqlite(pool) => {
                    sqlx::query_scalar(
                        "SELECT EXISTS(SELECT 1 FROM schema_migrations WHERE version = ?)",
                    )
                    .bind(version)
                    .fetch_one(pool)
                    .await?
                }
            };

            if !applied {
                tracing::info!("Applying migration: {}", version);
                migration.up(db).await?;
                match db.pool() {
                    DatabaseConnection::Postgres(pool) => {
                        sqlx::query("INSERT INTO schema_migrations (version) VALUES ($1)")
                            .bind(version)
                            .execute(pool)
                            .await?;
                    }
                    DatabaseConnection::Sqlite(pool) => {
                        sqlx::query("INSERT INTO schema_migrations (version) VALUES (?)")
                            .bind(version)
                            .execute(pool)
                            .await?;
                    }
                }
            }
        }
        Ok(())
    }

    /// Rollback the last migration.
    pub async fn rollback_last(&self, db: &Database) -> anyhow::Result<()> {
        self.ensure_table(db).await?;

        let version: Option<String> = match db.pool() {
            DatabaseConnection::Postgres(pool) => {
                sqlx::query_scalar(
                    "SELECT version FROM schema_migrations ORDER BY version DESC LIMIT 1",
                )
                .fetch_optional(pool)
                .await?
            }
            DatabaseConnection::Sqlite(pool) => {
                sqlx::query_scalar(
                    "SELECT version FROM schema_migrations ORDER BY version DESC LIMIT 1",
                )
                .fetch_optional(pool)
                .await?
            }
        };

        if let Some(version) = version {
            if let Some(migration) = self.migrations.iter().find(|m| m.name() == version) {
                tracing::info!("Rolling back migration: {}", version);
                migration.down(db).await?;
                match db.pool() {
                    DatabaseConnection::Postgres(pool) => {
                        sqlx::query("DELETE FROM schema_migrations WHERE version = $1")
                            .bind(&version)
                            .execute(pool)
                            .await?;
                    }
                    DatabaseConnection::Sqlite(pool) => {
                        sqlx::query("DELETE FROM schema_migrations WHERE version = ?")
                            .bind(&version)
                            .execute(pool)
                            .await?;
                    }
                }
            }
        }
        Ok(())
    }
}

impl Default for MigrationRunner {
    fn default() -> Self {
        Self::new()
    }
}
