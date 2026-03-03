# Core Concepts in Craken

Craken is built around several core concepts that provide a cohesive and type-safe development experience.

## Application Kernel (`App`)

The application kernel, defined in `craken-core`, is the heart of a Craken application. It manages the lifecycle of your application, from initialization and registration to booting and running.

### Service Providers

Service providers are used to register components into the application's dependency injection container. They provide a clear and modular way to configure your application's dependencies.

```rust
use craken_core::{App, ServiceProvider};
use craken_container::Container;

pub struct AppServiceProvider;

impl ServiceProvider for AppServiceProvider {
    fn register(&self, c: &mut Container) {
        c.register(MyService::new());
    }
}
```

## Dependency Injection (`craken-container`)

Craken features a robust, type-safe dependency injection system that catches errors at compile-time. Dependencies are resolved from the container using the `Inject<T>` extractor in your handlers.

### Singletons vs Scoped Dependencies

- **Singletons**: Registered once and shared across all requests.
- **Scoped**: A fresh instance is created for each incoming request.

## HTTP Routing (`craken-http`)

Routing in Craken is handled by the `HttpServer` and procedural macros in `craken-macros`.

### Controllers

Controllers are used to group related route handlers together. Use the `#[controller]` attribute to define a controller and `#[get]`, `#[post]`, etc., to define individual routes.

```rust
use craken_macros::{controller, get};
use craken_http::{Inject, CrakenError};
use axum::Json;

pub struct UserController;

#[controller]
impl UserController {
    #[get("/users")]
    pub async fn index(svc: Inject<UserService>) -> Result<Json<Vec<User>>, CrakenError> {
        Ok(Json(svc.0.all()))
    }
}
```

## Database & ORM (`craken-database`)

Craken provides a lightweight ORM-like system on top of `sqlx`. It uses the `Model` trait and `Repository<T>` abstraction to provide a clean and type-safe way to interact with your database.

### Models

Define your database models using the `#[derive(Model)]` macro.

```rust
use craken_database::Model;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow, Model, Clone)]
#[table("users")]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
}
```

### Repositories

Repositories wrap your models to provide high-level CRUD operations.

```rust
use craken_database::repository::Repository;

// Resolve a repository from the DI container
let repo: Inject<Repository<User>> = ...;
let user = repo.0.find(1).await?;
```

Next, learn how to [Contribute](../CONTRIBUTING.md) to Craken.
