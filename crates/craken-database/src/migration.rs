use async_trait::async_trait;
use crate::Database;
use sqlx::Postgres;

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
        Self { migrations: Vec::new() }
    }

    pub fn add(&mut self, migration: Box<dyn Migration>) {
        self.migrations.push(migration);
    }

    /// Ensure `schema_migrations` table exists.
    async fn ensure_table(&self, db: &Database) -> anyhow::Result<()> {
        sqlx::query("CREATE TABLE IF NOT EXISTS schema_migrations (version VARCHAR(255) PRIMARY KEY)")
            .execute(db.pool())
            .await?;
        Ok(())
    }

    /// Run all pending migrations.
    pub async fn run_pending(&self, db: &Database) -> anyhow::Result<()> {
        self.ensure_table(db).await?;

        for migration in &self.migrations {
            let version = migration.name();
            let applied: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM schema_migrations WHERE version = $1)")
                .bind(version)
                .fetch_one(db.pool())
                .await?;

            if !applied {
                tracing::info!("Applying migration: {}", version);
                migration.up(db).await?;
                sqlx::query("INSERT INTO schema_migrations (version) VALUES ($1)")
                    .bind(version)
                    .execute(db.pool())
                    .await?;
            }
        }
        Ok(())
    }

    /// Rollback the last migration.
    pub async fn rollback_last(&self, db: &Database) -> anyhow::Result<()> {
        self.ensure_table(db).await?;

        if let Some(version) = sqlx::query_scalar::<Postgres, String>("SELECT version FROM schema_migrations ORDER BY version DESC LIMIT 1")
            .fetch_optional(db.pool())
            .await? 
        {
            if let Some(migration) = self.migrations.iter().find(|m| m.name() == version) {
                tracing::info!("Rolling back migration: {}", version);
                migration.down(db).await?;
                sqlx::query("DELETE FROM schema_migrations WHERE version = $1")
                    .bind(version)
                    .execute(db.pool())
                    .await?;
            }
        }
        Ok(())
    }
}
