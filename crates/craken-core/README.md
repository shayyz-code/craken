# craken-core

The application kernel and core abstractions for the Craken framework.

## Features

- **Application Kernel**: Manage the application lifecycle from registration to boot.
- **Service Providers**: Encapsulate service registration logic for modularity.
- **Extensions Support**: Easily add third-party plugins and extensions.
- **DI Integration**: Integrated with `craken-container` for seamless dependency injection.

## Usage

```rust
use craken_core::{App, ServiceProvider};
use craken_container::Container;

pub struct MyServiceProvider;

impl ServiceProvider for MyServiceProvider {
    fn register(&self, container: &mut Container) {
        container.register(MyService::new());
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut app = App::new();
    
    // Register services
    app.register_services(&MyServiceProvider);
    
    // Boot application kernel
    app.boot().await?;
    
    // Use container
    let container = app.into_container();
    
    Ok(())
}
```

## Part of Craken

This crate is a core component of the [Craken Framework](https://github.com/shayyz-code/craken).
