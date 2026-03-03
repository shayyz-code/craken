# craken-macros

Procedural macros for the Craken framework to simplify routing and dependency injection.

## Macros

- **`#[controller]`**: Automates the implementation of `RouteProvider` for a struct.
- **`#[get]`, `#[post]`, `#[put]`, `#[delete]`, `#[patch]`**: Define HTTP routes within a controller or as standalone functions.
- **`#[derive(Model)]`**: Automatically implements database model traits.

## Usage

### Controller Scaffolding

```rust
use craken_macros::{controller, get};

pub struct UserController;

#[controller]
impl UserController {
    #[get("/users")]
    pub async fn index() -> &'static str {
        "User List"
    }
}
```

### Standalone Routes

```rust
use craken_macros::get;

#[get("/health")]
pub async fn health() -> &'static str {
    "OK"
}
```

## Part of Craken

This crate is a core component of the [Craken Framework](https://github.com/shayyz-code/craken).
