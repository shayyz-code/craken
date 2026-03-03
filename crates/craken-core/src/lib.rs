use std::sync::Arc;

use anyhow::Result;
use craken_container::Container;

/// Implement this trait to register services into the DI container during the
/// `register_services` phase of the application lifecycle.
///
/// # Example
///
/// ```rust,ignore
/// pub struct AppServiceProvider;
///
/// impl ServiceProvider for AppServiceProvider {
///     fn register(&self, container: &mut Container) {
///         container.register(MyService::new());
///         container.register_scoped(|| RequestScopedService::new());
///     }
/// }
/// ```
pub trait ServiceProvider: Send + Sync {
    fn register(&self, container: &mut Container);
}

/// The Application Kernel — the central orchestrator for the Craken lifecycle.
///
/// # Lifecycle
///
/// ```text
/// App::new()
///   └─ register_services(&provider)   ← wire up DI bindings
///   └─ boot().await                   ← initialise core infrastructure
///   └─ into_container()               ← hand off Arc<Container> to HttpServer
/// ```
pub struct App {
    container: Container,
}

impl App {
    pub fn new() -> Self {
        Self {
            container: Container::new(),
        }
    }

    /// Register all services declared by `provider` into the container.
    ///
    /// May be called multiple times with different providers to compose
    /// service registrations from separate modules.
    pub fn register_services(&mut self, provider: &dyn ServiceProvider) -> &mut Self {
        provider.register(&mut self.container);
        self
    }

    /// Direct access to the container for one-off manual registrations.
    pub fn container(&mut self) -> &mut Container {
        &mut self.container
    }

    /// Read-only view of the container (e.g., for extensions that inspect
    /// registered services during boot).
    pub fn get_container(&self) -> &Container {
        &self.container
    }

    /// Boot the application kernel.
    ///
    /// Called after all `register_services` calls. Use this phase to
    /// validate required registrations, load environment config, or
    /// run any async initialisation that services depend on.
    pub async fn boot(&mut self) -> Result<()> {
        Ok(())
    }

    /// Consume the kernel and return the finalised container wrapped in [`Arc`].
    ///
    /// The resulting `Arc<Container>` is injected into every request handler
    /// as axum router state.
    pub fn into_container(self) -> Arc<Container> {
        Arc::new(self.container)
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

/// A trait for optional framework extensions (plugins).
///
/// Extensions integrate third-party components (e.g., ORM, auth) without
/// coupling them to the core lifecycle.
#[async_trait::async_trait]
pub trait Extension: Send + Sync {
    /// Register extension-owned services into the container.
    async fn register(&self, app: &mut App) -> Result<()>;

    /// Perform post-registration initialisation (e.g., run migrations).
    async fn boot(&self, app: &App) -> Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockService;
    struct MyProvider;
    impl ServiceProvider for MyProvider {
        fn register(&self, container: &mut Container) {
            container.register(MockService);
        }
    }

    #[test]
    fn test_app_registration() {
        let mut app = App::new();
        app.register_services(&MyProvider);
        assert!(app.get_container().resolve::<MockService>().is_some());
    }
}
