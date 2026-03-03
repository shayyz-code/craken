use std::sync::Arc;

use async_trait::async_trait;
use axum::extract::FromRequestParts;
use craken_container::{Container, ScopedContainer};
use http::request::Parts;

/// Per-request dependency context backed by a [`ScopedContainer`].
///
/// Add `ctx: RequestContext` to any handler signature to get access to
/// both scoped and singleton services for the duration of that request.
///
/// Axum resolves this automatically from the router state (`Arc<Container>`):
/// a fresh [`ScopedContainer`] is created for each request, so scoped
/// services see their own private instance while singletons are shared.
///
/// # Example
///
/// ```rust,ignore
/// async fn handler(ctx: RequestContext) -> Result<Json<String>, CrakenError> {
///     let svc = ctx.resolve::<UserService>()
///         .ok_or_else(|| CrakenError::internal("UserService not registered"))?;
///     Ok(Json(svc.greet()))
/// }
/// ```
pub struct RequestContext {
    pub container: Arc<ScopedContainer>,
}

impl RequestContext {
    /// Resolve a service from the request-scoped container.
    ///
    /// Checks the scoped cache first, then instantiates via a registered
    /// scoped factory, then falls back to the root singleton.
    pub fn resolve<T: Send + Sync + 'static>(&self) -> Option<Arc<T>> {
        self.container.resolve::<T>()
    }

    /// Resolve a singleton directly from the root container, bypassing the
    /// scoped layer. Use when you explicitly want the shared instance.
    pub fn resolve_singleton<T: Send + Sync + 'static>(&self) -> Option<Arc<T>> {
        self.container.resolve_singleton::<T>()
    }
}

#[async_trait]
impl FromRequestParts<Arc<Container>> for RequestContext {
    /// [`RequestContext`] extraction is infallible — a fresh scope is always
    /// constructed from the router state, even if it contains no services.
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(
        _parts: &mut Parts,
        state: &Arc<Container>,
    ) -> Result<Self, Self::Rejection> {
        Ok(RequestContext {
            container: Arc::new(ScopedContainer::new(Arc::clone(state))),
        })
    }
}
