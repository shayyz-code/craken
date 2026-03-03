# craken-http

HTTP server integration and routing for the Craken framework.

## Features

- **Built on Axum**: Leverages the power and performance of `axum`.
- **Integrated DI**: Automatic dependency injection in route handlers with `Inject<T>`.
- **Request Scoped Context**: Easy access to request-scoped dependencies.
- **Middleware Support**: Flexible middleware stack for custom logic.
- **Error Handling**: Standardized error responses and types.

## Usage

```rust
use craken_http::{HttpServer, LoggingMiddleware, Inject};
use craken_macros::{controller, get};
use std::sync::Arc;

pub struct MyController;

#[controller]
impl MyController {
    #[get("/items")]
    pub async fn index(svc: Inject<MyService>) -> &'static str {
        "Items List"
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    HttpServer::new()
        .with_middleware(LoggingMiddleware)
        .configure_routes(&MyController)
        .run(container, "127.0.0.1:8080")
        .await?;
    
    Ok(())
}
```

## Part of Craken

This crate is a core component of the [Craken Framework](https://github.com/shayyz-code/craken).
