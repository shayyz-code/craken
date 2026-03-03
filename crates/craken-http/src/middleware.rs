use std::sync::Arc;
use std::time::Instant;

use axum::{extract::Request, middleware::Next, response::IntoResponse, Router};
use craken_container::Container;

/// Core middleware abstraction for Craken.
///
/// A [`CrakenMiddleware`] wraps a `Router<Arc<Container>>` with an additional
/// Tower layer. Implementations call `router.layer(...)` using axum's
/// `middleware::from_fn` (or any Tower-compatible layer) and return the
/// wrapped router.
///
/// # Example — simple auth gate
///
/// ```rust,ignore
/// pub struct AuthMiddleware { secret: String }
///
/// impl CrakenMiddleware for AuthMiddleware {
///     fn apply(&self, router: Router<Arc<Container>>) -> Router<Arc<Container>> {
///         let secret = self.secret.clone();
///         router.layer(axum::middleware::from_fn(move |req, next| {
///             let secret = secret.clone();
///             async move { check_token(req, next, &secret).await }
///         }))
///     }
/// }
/// ```
pub trait CrakenMiddleware: Send + Sync + 'static {
    fn apply(&self, router: Router<Arc<Container>>) -> Router<Arc<Container>>;
}

/// An ordered stack of [`CrakenMiddleware`] implementations.
///
/// Middlewares are applied in push order: the first pushed is the outermost
/// layer (first to see the request, last to see the response).
#[derive(Default)]
pub struct MiddlewareStack {
    middlewares: Vec<Box<dyn CrakenMiddleware>>,
}

impl MiddlewareStack {
    pub fn new() -> Self {
        Self::default()
    }

    /// Push a middleware onto the stack.
    pub fn push(mut self, middleware: impl CrakenMiddleware) -> Self {
        self.middlewares.push(Box::new(middleware));
        self
    }

    /// Apply all registered middlewares to `router` in registration order.
    pub(crate) fn apply(self, mut router: Router<Arc<Container>>) -> Router<Arc<Container>> {
        for middleware in self.middlewares {
            router = middleware.apply(router);
        }
        router
    }
}

// ── Built-in middleware ───────────────────────────────────────────────────────

/// Logs every inbound request and outbound response using the `tracing` crate.
///
/// Emits two structured log events per request:
/// - `→ request` with method and path.
/// - `← response` with status code and elapsed milliseconds.
///
/// # Setup
///
/// ```rust,ignore
/// HttpServer::new()
///     .with_middleware(LoggingMiddleware)
///     // ...
/// ```
pub struct LoggingMiddleware;

impl CrakenMiddleware for LoggingMiddleware {
    fn apply(&self, router: Router<Arc<Container>>) -> Router<Arc<Container>> {
        router.layer(axum::middleware::from_fn(logging_fn))
    }
}

async fn logging_fn(req: Request, next: Next) -> impl IntoResponse {
    let method = req.method().clone();
    let path = req.uri().path().to_owned();
    let start = Instant::now();

    tracing::info!(method = %method, path = %path, "→ request");

    let response = next.run(req).await;
    let elapsed = start.elapsed();

    tracing::info!(
        method = %method,
        path   = %path,
        status = %response.status().as_u16(),
        elapsed_ms = elapsed.as_millis(),
        "← response"
    );

    response
}
