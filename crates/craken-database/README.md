# craken-database

SQLx-powered database abstraction and ORM for the Craken framework.

## Features

- **Built on SQLx**: Asynchronous, compile-time verified queries for PostgreSQL.
- **Repository Pattern**: Abstract data access logic into clean, reusable repositories.
- **Model Support**: Simple trait-based model definitions with `#[derive(Model)]`.
- **Migration Management**: Unified migration runner for managing database schemas.
- **DI Integration**: Automatically inject database connections and repositories into handlers.

## Usage

### Define a Model

```rust
use craken_database::Model;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow, Model, Clone)]
#[table("users")]
pub struct User {
    pub id: i64,
    pub name: String,
}
```

### Use a Repository

```rust
use craken_database::repository::Repository;

let repo = Repository::<User>::new(db.clone());
let users = repo.all().await?;
```

## Part of Craken

This crate is a core component of the [Craken Framework](https://github.com/shayyz-code/craken).
