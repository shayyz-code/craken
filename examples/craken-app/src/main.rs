//! Craken example — demonstrates the full framework surface area:
//!
//! - `#[controller]` + `#[get]` / `#[post]` / `#[delete]` for routing
//! - `Inject<T>` for singleton DI in handlers
//! - `RequestContext` for per-request scoped DI
//! - `#[derive(Model)]` + `Repository<T>` for database access
//! - `LoggingMiddleware` wrapping all routes

use axum::{extract::Path, http::StatusCode, Json};
use clap::Parser;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::sync::Arc;

use craken_cli::{Cli, Commands};
use craken_container::Container;
use craken_core::{App, ServiceProvider};
use craken_database::{repository::Repository, Database, Model};
use craken_http::{CrakenError, HttpServer, Inject, LoggingMiddleware, RequestContext};
use craken_macros::{controller, get};

// ── Domain types ──────────────────────────────────────────────────────────────

/// Database Model
#[derive(Debug, Serialize, FromRow, Model, Clone)]
#[table("users")]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
}

// ── Services ──────────────────────────────────────────────────────────────────

/// Scoped — a fresh instance per request, created by the factory below.
pub struct RequestTracker {
    pub request_id: String,
}

impl RequestTracker {
    pub fn new() -> Self {
        let nano = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.subsec_nanos())
            .unwrap_or(0);
        Self {
            request_id: format!("req-{nano}"),
        }
    }
}

impl Default for RequestTracker {
    fn default() -> Self {
        Self::new()
    }
}

// ── Service registration ──────────────────────────────────────────────────────

pub struct AppServiceProvider {
    db: Arc<Database>,
}

impl AppServiceProvider {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }
}

impl ServiceProvider for AppServiceProvider {
    fn register(&self, c: &mut Container) {
        c.register_arc(self.db.clone());
        c.register(Repository::<User>::new(self.db.clone()));
        c.register_scoped(RequestTracker::new);
    }
}

// ── UserController ────────────────────────────────────────────────────────────

pub struct UserController;

#[controller]
impl UserController {
    /// GET /users — list all users using Repository.
    #[get("/users")]
    pub async fn index(repo: Inject<Repository<User>>) -> Result<Json<Vec<User>>, CrakenError> {
        let users = repo.0.all().await?;
        Ok(Json(users))
    }

    /// GET /users/:id — fetch one user.
    #[get("/users/:id")]
    pub async fn show(Path(id): Path<i64>, ctx: RequestContext) -> Result<Json<User>, CrakenError> {
        let tracker = ctx
            .resolve::<RequestTracker>()
            .ok_or_else(|| CrakenError::internal("RequestTracker not registered"))?;

        tracing::info!(
            request_id = %tracker.request_id,
            user_id    = id,
            "handling show"
        );

        let repo = ctx
            .resolve::<Repository<User>>()
            .ok_or_else(|| CrakenError::internal("Repository<User> not registered"))?;

        repo.find(id).await.map(Json)
    }

    /// POST /users — create a user (returns 201 Created).
    #[post("/users")]
    pub async fn create() -> StatusCode {
        StatusCode::CREATED
    }
}

// ── Standalone route via #[get] ───────────────────────────────────────────────

#[get("/health")]
pub async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({ "status": "ok" }))
}

// ── Entry point ───────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    craken_logging::Logging::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Serve { addr } => {
            let mut app = App::new();

            // In a real app, this URL would come from config.
            let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
                "postgres://postgres:password@localhost/craken_app".to_string()
            });

            // Initialize Database
            let db = Arc::new(Database::connect(&db_url).await?);

            // Register services
            app.register_services(&AppServiceProvider::new(db));

            app.boot().await?;

            println!("🦀 Craken framework starting on http://{}", addr);

            HttpServer::new()
                .with_middleware(LoggingMiddleware)
                .configure_routes(&UserController)
                .configure_routes(&HealthRoute)
                .run(app.into_container(), &addr)
                .await?;
        }
        Commands::New { name, .. } => {
            println!("Scaffolding new project: {}...", name);
        }
        Commands::Dev { addr } => {
            println!("Starting dev server on {}...", addr);
        }
        _ => {
            println!("Command not implemented in this example.");
        }
    }

    Ok(())
}
