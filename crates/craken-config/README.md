# craken-config

Hierarchical configuration management for the Craken framework.

## Features

- **Hierarchical Configuration**: Manage complex configuration structures with ease.
- **Support for Multiple Formats**: 
  - **TOML**
  - **JSON**
  - **YAML**
  - **INI**
- **Environment Variable Overrides**: Override configuration values with environment variables.
- **Type-safe Access**: Access configuration values with Rust's type system.

## Usage

```rust
use craken_config::Config;
use serde::Deserialize;

#[derive(Deserialize)]
struct AppConfig {
    port: u16,
    db_url: String,
}

// Load configuration from files and environment
let config: AppConfig = Config::load()?;

println!("Server running on port: {}", config.port);
```

## Part of Craken

This crate is a core component of the [Craken Framework](https://github.com/shayyz-code/craken).
