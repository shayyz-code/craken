use std::sync::Arc;

use axum::Router;
use craken_container::Container;

use crate::middleware::{CrakenMiddleware, MiddlewareStack};

/// Implement this trait to declare your application's route tree.
///
/// The returned `Router` uses `Arc<Container>` as axum state so that
/// [`Inject<T>`] and [`RequestContext`] extractors work automatically in
/// every handler.
///
/// # Example
///
/// ```rust,ignore
/// pub struct ApiRoutes;
///
/// impl RouteProvider for ApiRoutes {
///     fn routes(&self) -> Router<Arc<Container>> {
///         Router::new()
///             .route("/users",     get(list_users))
///             .route("/users/:id", get(get_user))
///     }
/// }
/// ```
pub trait RouteProvider: Send + Sync {
    fn routes(&self) -> Router<Arc<Container>>;
}

/// The Craken HTTP server.
///
/// Collects routes from [`RouteProvider`] implementations, stacks Tower
/// middleware via [`CrakenMiddleware`], injects the DI container as axum
/// state, and starts serving.
///
/// # Lifecycle
///
/// ```text
/// HttpServer::new()
///   .with_middleware(LoggingMiddleware)
///   .configure_routes(&MyRoutes)
///   .run(container, "0.0.0.0:8080")
///   .await?
/// ```
pub struct HttpServer {
    router: Router<Arc<Container>>,
    middleware: MiddlewareStack,
}

impl HttpServer {
    pub fn new() -> Self {
        Self {
            router: Router::new(),
            middleware: MiddlewareStack::new(),
        }
    }

    /// Register a raw axum route directly on the server.
    pub fn route(
        mut self,
        path: &str,
        method_router: axum::routing::MethodRouter<Arc<Container>>,
    ) -> Self {
        self.router = self.router.route(path, method_router);
        self
    }

    /// Mount all routes returned by `provider`.
    ///
    /// May be called multiple times to compose routes from separate modules.
    pub fn configure_routes(mut self, provider: &dyn RouteProvider) -> Self {
        self.router = self.router.merge(provider.routes());
        self
    }

    /// Add a middleware to the stack.
    ///
    /// Middlewares are applied in push order (first pushed = outermost layer).
    pub fn with_middleware(mut self, middleware: impl CrakenMiddleware) -> Self {
        self.middleware = self.middleware.push(middleware);
        self
    }

    /// Merge an additional pre-built axum router.
    pub fn merge(mut self, other: Router<Arc<Container>>) -> Self {
        self.router = self.router.merge(other);
        self
    }

    /// Bind to `addr`, apply middleware, inject the DI container, and start
    /// the async HTTP server. Runs until the process is killed or an error occurs.
    pub async fn run(self, container: Arc<Container>, addr: &str) -> anyhow::Result<()> {
        let router = self.middleware.apply(self.router).with_state(container);

        let listener = tokio::net::TcpListener::bind(addr).await?;
        tracing::info!(addr = %addr, "HTTP server listening");
        axum::serve(listener, router).await?;
        Ok(())
    }
}

impl Default for HttpServer {
    fn default() -> Self {
        Self::new()
    }
}
