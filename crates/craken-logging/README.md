# craken-logging

Structured, asynchronous logging for the Craken framework.

## Features

- **Asynchronous Logging**: Perform logging without blocking the main execution thread.
- **Structured Output**: Log in structured formats (like JSON) for better observability.
- **Configurable Filters**: Fine-grained control over log levels and modules.
- **Easy Integration**: Initialize logging with a single function call.

## Usage

Initialize logging in your `main.rs`:

```rust
use craken_logging::Logging;

fn main() {
    Logging::init();
    
    tracing::info!("Application started");
}
```

## Part of Craken

This crate is a core component of the [Craken Framework](https://github.com/shayyz-code/craken).
