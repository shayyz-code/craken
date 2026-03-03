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
        fs::read_to_string(&mod_path)
            .with_context(|| format!("Failed to read {mod_path:?}"))?
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

    fs::write(&mod_path, &updated)
        .with_context(|| format!("Failed to write {mod_path:?}"))?;

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
//   HttpServer::new().configure_routes(&{name})

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
        Inject(_svc): Inject<{name}Service>,
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

// ── `craken new <name>` ───────────────────────────────────────────────────────

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
