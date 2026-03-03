# craken-container

Compile-time safe dependency injection container for the Craken framework.

## Features

- **Type-safe Dependency Injection**: Leverage Rust's type system to manage dependencies.
- **Support for Multiple Lifetimes**: 
  - **Singleton**: Single instance shared across the application.
  - **Scoped**: Fresh instance per request.
- **Easy Registration**: Simple API for registering instances and factories.

## Usage

```rust
use craken_container::Container;

let mut container = Container::new();

// Register a singleton
container.register(MyService::new());

// Resolve a dependency
if let Some(service) = container.resolve::<MyService>() {
    // Use service
}
```

## Part of Craken

This crate is a core component of the [Craken Framework](https://github.com/shayyz-code/craken).
