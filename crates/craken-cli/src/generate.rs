use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};

// ── Name utilities ────────────────────────────────────────────────────────────

/// `UserController` → `user_controller`
pub fn pascal_to_snake(name: &str) -> String {
    let mut out = String::new();
    for (i, ch) in name.char_indices() {
        if ch.is_uppercase() && i > 0 {
            out.push('_');
        }
        out.extend(ch.to_lowercase());
    }
    out
}

// ── Shared helpers ────────────────────────────────────────────────────────────

/// Ensure `dir` exists and append `pub mod <module_name>;` to `dir/mod.rs`
/// (creating the file if absent, skipping if the line already exists).
fn register_in_mod(dir: &Path, module_name: &str) -> Result<()> {
    fs::create_dir_all(dir)?;

    let mod_path = dir.join("mod.rs");
    let declaration = format!("pub mod {module_name};");

    let existing = if mod_path.exists() {
        fs::read_to_string(&mod_path).with_context(|| format!("Failed to read {mod_path:?}"))?
    } else {
        String::new()
    };

    if existing.contains(&declaration) {
        return Ok(()); // already registered
    }

    let mut updated = existing;
    if !updated.is_empty() && !updated.ends_with('\n') {
        updated.push('\n');
    }
    updated.push_str(&declaration);
    updated.push('\n');

    fs::write(&mod_path, &updated).with_context(|| format!("Failed to write {mod_path:?}"))?;

    Ok(())
}

// ── `craken make controller <Name>` ──────────────────────────────────────────

/// Scaffold `src/controllers/<snake_name>.rs` and register it in `src/controllers/mod.rs`.
pub fn make_controller(name: &str) -> Result<()> {
    let snake = pascal_to_snake(name);
    let dir = Path::new("src/controllers");
    fs::create_dir_all(dir)?;

    let file = dir.join(format!("{snake}.rs"));
    if file.exists() {
        anyhow::bail!("Controller already exists: {file:?}");
    }

    let template = format!(
        r#"use axum::{{extract::Path, Json}};
use craken_http::{{CrakenError, Inject, RequestContext}};
use craken_macros::{{controller, delete, get, post, put}};
use serde::{{Deserialize, Serialize}};

// ── Replace with your domain types ───────────────────────────────────────────

#[derive(Serialize)]
pub struct Item {{
    pub id: u64,
}}

pub struct {name};

// ── Route handlers ────────────────────────────────────────────────────────────
//
// Methods annotated with #[get], #[post], etc. must NOT take a `self`
// receiver — use Inject<T> or RequestContext to access services.
//
// Mount this controller:
//   HttpServer::new().configure_routes(&controllers::{snake}::{name})

#[controller]
impl {name} {{
    /// GET /
    #[get("/")]
    pub async fn index() -> Result<Json<Vec<Item>>, CrakenError> {{
        Ok(Json(vec![]))
    }}

    /// GET /:id
    #[get("/:id")]
    pub async fn show(Path(id): Path<u64>) -> Result<Json<Item>, CrakenError> {{
        Ok(Json(Item {{ id }}))
    }}
}}
"#
    );

    fs::write(&file, &template).with_context(|| format!("Failed to write {file:?}"))?;
    register_in_mod(dir, &snake)?;

    println!("✓  src/controllers/{snake}.rs");
    println!("   Register: HttpServer::new().configure_routes(&controllers::{snake}::{name})");
    Ok(())
}

// ── `craken make service <Name>` ─────────────────────────────────────────────

/// Scaffold `src/services/<snake_name>.rs` and register it in `src/services/mod.rs`.
pub fn make_service(name: &str) -> Result<()> {
    let snake = pascal_to_snake(name);
    let dir = Path::new("src/services");
    fs::create_dir_all(dir)?;

    let file = dir.join(format!("{snake}.rs"));
    if file.exists() {
        anyhow::bail!("Service already exists: {file:?}");
    }

    let template = format!(
        r#"/// {name} — application-layer service.
///
/// Register as a singleton:
/// ```rust,ignore
/// container.register({name}::new());
/// ```
///
/// Register as scoped (one instance per request):
/// ```rust,ignore
/// container.register_scoped({name}::new);
/// ```
pub struct {name};

impl {name} {{
    pub fn new() -> Self {{
        Self
    }}
}}

impl Default for {name} {{
    fn default() -> Self {{
        Self::new()
    }}
}}
"#
    );

    fs::write(&file, &template).with_context(|| format!("Failed to write {file:?}"))?;
    register_in_mod(dir, &snake)?;

    println!("✓  src/services/{snake}.rs");
    println!("   Register: container.register(services::{snake}::{name}::new())");
    Ok(())
}

// ── `craken make module <Name>` ──────────────────────────────────────────────

/// Scaffold `src/modules/<snake_name>/` with `mod.rs`, `controller.rs`,
/// and `service.rs`, and register the module in `src/modules/mod.rs`.
pub fn make_module(name: &str) -> Result<()> {
    let snake = pascal_to_snake(name);
    let module_dir = PathBuf::from(format!("src/modules/{snake}"));

    if module_dir.exists() {
        anyhow::bail!("Module already exists: {module_dir:?}");
    }

    fs::create_dir_all(&module_dir)?;

    // mod.rs
    let mod_template = format!(
        r#"pub mod controller;
pub mod service;

pub use controller::{name}Controller;
pub use service::{name}Service;
"#
    );
    fs::write(module_dir.join("mod.rs"), &mod_template)?;

    // controller.rs
    let controller_template = format!(
        r#"use axum::Json;
use craken_http::{{CrakenError, Inject}};
use craken_macros::{{controller, get, post}};
use super::service::{name}Service;

pub struct {name}Controller;

#[controller]
impl {name}Controller {{
    #[get("/{snake}")]
    pub async fn index(
        _svc: Inject<{name}Service>,
    ) -> Result<Json<Vec<serde_json::Value>>, CrakenError> {{
        Ok(Json(vec![]))
    }}
}}
"#
    );
    fs::write(module_dir.join("controller.rs"), &controller_template)?;

    // service.rs
    let service_template = format!(
        r#"pub struct {name}Service;

impl {name}Service {{
    pub fn new() -> Self {{
        Self
    }}
}}

impl Default for {name}Service {{
    fn default() -> Self {{
        Self::new()
    }}
}}
"#
    );
    fs::write(module_dir.join("service.rs"), &service_template)?;

    // Register in src/modules/mod.rs
    register_in_mod(Path::new("src/modules"), &snake)?;

    println!("✓  src/modules/{snake}/mod.rs");
    println!("   src/modules/{snake}/controller.rs");
    println!("   src/modules/{snake}/service.rs");
    println!("   Registered in src/modules/mod.rs");
    Ok(())
}

// ── `craken make migration <name>` ──────────────────────────────────────────

/// Scaffold `migrations/<timestamp>_<snake_name>.rs`.
pub fn make_migration(name: &str) -> Result<()> {
    let snake = pascal_to_snake(name);
    let dir = Path::new("src/migrations");
    fs::create_dir_all(dir)?;

    let timestamp = chrono::Local::now().format("%Y%m%d%H%M%S");
    let version = format!("{}_{}", timestamp, snake);
    let file = dir.join(format!("{}.rs", version));

    if file.exists() {
        anyhow::bail!("Migration already exists: {file:?}");
    }

    let template = format!(
        r#"use craken_database::{{async_trait::async_trait, migration::Migration, Database, DatabaseConnection}};
use sqlx;

pub struct {name};

#[async_trait]
impl Migration for {name} {{
    fn name(&self) -> &'static str {{
        "{version}"
    }}

    async fn up(&self, db: &Database) -> anyhow::Result<()> {{
        // Write your migration UP logic here
        // sqlx::query("CREATE TABLE IF NOT EXISTS ...").execute(db.pool()).await?;
        Ok(())
    }}

    async fn down(&self, db: &Database) -> anyhow::Result<()> {{
        // Write your migration DOWN logic here
        // sqlx::query("DROP TABLE IF EXISTS ...").execute(db.pool()).await?;
        Ok(())
    }}
}}
"#
    );

    fs::write(&file, &template).with_context(|| format!("Failed to write {file:?}"))?;
    register_in_mod(dir, &version)?;

    println!("✓  src/migrations/{}.rs", version);
    Ok(())
}

// ── `craken new <name>` ───────────────────────────────────────────────────────

/// Scaffold a new project structure in `./<name>`.
pub fn make_app(name: &str, db_type: &str) -> Result<()> {
    let root = Path::new(name);
    if root.exists() {
        anyhow::bail!("Directory already exists: {root:?}");
    }

    fs::create_dir_all(root.join("src/controllers"))?;
    fs::create_dir_all(root.join("src/services"))?;
    fs::create_dir_all(root.join("src/migrations"))?;
    fs::create_dir_all(root.join("src/models"))?;
    fs::create_dir_all(root.join("src/services"))?;
    fs::write(root.join("src/services/mod.rs"), "")?;
    fs::create_dir_all(root.join("config"))?;

    // src/models/user.rs
    let user_model = r#"use craken_database::Model;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow, Model, Clone)]
#[table("users")]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
}
"#;
    fs::write(root.join("src/models/user.rs"), user_model)?;

    // src/models/mod.rs
    fs::write(root.join("src/models/mod.rs"), "pub mod user;\n")?;

    // Cargo.toml
    let cargo_toml = format!(
        r#"[package]
name = "{name}"
version = "0.1.0"
edition = "2021"

[dependencies]
craken-core = {{ git = "https://github.com/shayyz-code/craken.git" }}
craken-http = {{ git = "https://github.com/shayyz-code/craken.git" }}
craken-container = {{ git = "https://github.com/shayyz-code/craken.git" }}
craken-database = {{ git = "https://github.com/shayyz-code/craken.git" }}
craken-macros = {{ git = "https://github.com/shayyz-code/craken.git" }}
craken-logging = {{ git = "https://github.com/shayyz-code/craken.git" }}
tokio = {{ version = "1.0", features = ["full"] }}
axum = "0.7"
serde = {{ version = "1.0", features = ["derive"] }}
anyhow = "1.0"
sqlx = {{ version = "0.7", features = ["runtime-tokio-native-tls", "postgres", "sqlite"] }}
"#
    );
    fs::write(root.join("Cargo.toml"), cargo_toml)?;

    // src/main.rs
    let main_rs = r#"use std::sync::Arc;
use craken_core::{App, ServiceProvider};
use craken_container::Container;
use craken_http::{HttpServer, LoggingMiddleware};
use craken_database::{Database, migration::MigrationRunner};
use craken_macros::get;
use crate::controllers::user_controller::UserController;

// ── App Modules ──────────────────────────────────────────────────────────────

mod migrations;
mod controllers;
mod services;
mod models;

// ── Simple Health Route ──────────────────────────────────────────────────────

#[get("/health")]
pub async fn health() -> &'static str {
    "OK"
}

// ── Service Registration ─────────────────────────────────────────────────────

pub struct AppServiceProvider {
    db: Arc<Database>,
}

impl ServiceProvider for AppServiceProvider {
    fn register(&self, c: &mut Container) {
        c.register_arc(self.db.clone());
        // Register your application services here
    }
}

// ── Application Entry Point ──────────────────────────────────────────────────

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    craken_logging::Logging::init();

    // Simple argument handling without manual clap in main.rs
    let args: Vec<String> = std::env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("serve");

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Arc::new(Database::connect(&db_url).await?);

    match command {
        "serve" => {
            let mut app = App::new();
            app.register_services(&AppServiceProvider { db: db.clone() });
            app.boot().await?;

            let addr = args.get(2).map(|s| s.as_str()).unwrap_or("127.0.0.1:8080");
            println!("🦀 Craken application starting on http://{}", addr);

            HttpServer::new()
                .with_middleware(LoggingMiddleware)
                .configure_routes(&HealthRoute)
                .configure_routes(&UserController)
                .run(app.into_container(), addr)
                .await?;
        }
        "migrate" => {
            let mut runner = MigrationRunner::new();
            runner.add(Box::new(migrations::m20231012_create_users::CreateUsers));
            runner.run_pending(&db).await?;
            println!("✓  Migrations complete");
        }
        "rollback" => {
            let mut runner = MigrationRunner::new();
            runner.add(Box::new(migrations::m20231012_create_users::CreateUsers));
            runner.rollback_last(&db).await?;
            println!("✓  Rollback complete");
        }
        _ => {
            println!("Unknown command. Use: serve, migrate, rollback");
        }
    }
    Ok(())
}
"#;
    fs::write(root.join("src/main.rs"), main_rs)?;

    // .env
    let db_url = if db_type == "sqlite" {
        "DATABASE_URL=sqlite:db.sqlite"
    } else {
        "DATABASE_URL=postgres://postgres:password@localhost/my_app"
    };
    fs::write(root.join(".env"), format!("{}\n", db_url))?;

    // src/controllers/user_controller.rs
    let user_controller = r#"use craken_http::{{CrakenError}};
use craken_macros::{{controller}};

pub struct UserController;

#[controller]
impl UserController {}
"#;
    fs::write(
        root.join("src/controllers/user_controller.rs"),
        user_controller,
    )?;

    // src/controllers/mod.rs
    fs::write(
        root.join("src/controllers/mod.rs"),
        "pub mod user_controller;\n",
    )?;

    // src/migrations/m20231012_create_users.rs
    let user_migration = r#"use craken_database::{async_trait::async_trait, migration::Migration, Database, DatabaseConnection};
use sqlx;

pub struct CreateUsers;

#[async_trait]
impl Migration for CreateUsers {
    fn name(&self) -> &'static str {
        "20231012_create_users"
    }

    async fn up(&self, db: &Database) -> anyhow::Result<()> {
        let query = match db.pool() {
            DatabaseConnection::Postgres(_) => {
                "CREATE TABLE IF NOT EXISTS users (id BIGSERIAL PRIMARY KEY, name VARCHAR(255) NOT NULL, email VARCHAR(255) NOT NULL UNIQUE)"
            }
            DatabaseConnection::Sqlite(_) => {
                "CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL, email TEXT NOT NULL UNIQUE)"
            }
        };
        match db.pool() {
            DatabaseConnection::Postgres(pool) => {
                sqlx::query(query).execute(pool).await?;
            }
            DatabaseConnection::Sqlite(pool) => {
                sqlx::query(query).execute(pool).await?;
            }
        }
        Ok(())
    }

    async fn down(&self, db: &Database) -> anyhow::Result<()> {
        match db.pool() {
            DatabaseConnection::Postgres(pool) => {
                sqlx::query("DROP TABLE IF EXISTS users").execute(pool).await?;
            }
            DatabaseConnection::Sqlite(pool) => {
                sqlx::query("DROP TABLE IF EXISTS users").execute(pool).await?;
            }
        }
        Ok(())
    }
}
"#;
    fs::write(
        root.join("src/migrations/m20231012_create_users.rs"),
        user_migration,
    )?;

    // src/migrations/mod.rs
    fs::write(
        root.join("src/migrations/mod.rs"),
        "pub mod m20231012_create_users;\n",
    )?;

    println!("✓  Created new Craken project in: {}", name);
    println!("   Next steps:");
    println!("     cd {}", name);
    println!("     cargo run -- serve");

    Ok(())
}

/// Scaffold a brand-new Craken project with the standard directory layout.
///
/// ```text
/// <name>/
///   Cargo.toml
///   src/
///     main.rs
///     controllers/mod.rs
///     services/mod.rs
///     modules/mod.rs
///     middleware/mod.rs
///     models/mod.rs
/// ```
pub fn scaffold_project(name: &str) -> Result<()> {
    let root = Path::new(name);
    if root.exists() {
        anyhow::bail!("Directory '{name}' already exists");
    }

    // Directory tree
    for sub in &[
        "src/controllers",
        "src/services",
        "src/modules",
        "src/middleware",
        "src/models",
    ] {
        fs::create_dir_all(root.join(sub))?;
    }

    // Stub mod.rs files
    let mod_stub = "// Auto-generated by Craken. Add `pub mod` declarations here.\n";
    for sub in &["controllers", "services", "modules", "middleware", "models"] {
        fs::write(root.join(format!("src/{sub}/mod.rs")), mod_stub)?;
    }

    // Cargo.toml
    let cargo = format!(
        r#"[package]
name    = "{name}"
version = "0.1.0"
edition = "2021"

[dependencies]
craken-core    = "0.1"
craken-http    = "0.1"
craken-macros  = "0.1"
craken-logging = "0.1"
tokio          = {{ version = "1.0", features = ["full"] }}
axum           = "0.7"
serde          = {{ version = "1.0", features = ["derive"] }}
serde_json     = "1.0"
anyhow         = "1.0"
tracing        = "0.1"
"#
    );
    fs::write(root.join("Cargo.toml"), cargo)?;

    // src/main.rs
    let main_rs = r#"mod controllers;
mod services;
mod modules;
mod middleware;
mod models;

use craken_core::App;
use craken_http::{HttpServer, LoggingMiddleware};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    craken_logging::Logging::init();

    let mut app = App::new();
    app.boot().await?;
    // app.register_services(&services::AppServiceProvider);

    let container = app.into_container();

    HttpServer::new()
        .with_middleware(LoggingMiddleware)
        // .configure_routes(&controllers::YourController)
        .run(container, "127.0.0.1:8080")
        .await?;

    Ok(())
}
"#;
    fs::write(root.join("src/main.rs"), main_rs)?;

    println!("✓  Created project '{name}'");
    println!("   cd {name} && cargo run -- serve");
    Ok(())
}
