use std::sync::Arc;

use async_trait::async_trait;
use axum::extract::FromRequestParts;
use craken_container::Container;
use http::request::Parts;

use crate::error::CrakenError;

/// Type-safe singleton injector for axum handler parameters.
///
/// `Inject<T>` resolves `T` from the root [`Container`] (singletons only).
/// For scoped services use [`RequestContext`] instead.
///
/// Returns `500 Internal Server Error` if `T` was not registered.
///
/// # Example
///
/// ```rust,ignore
/// async fn list_users(
///     Inject(svc): Inject<UserService>,
/// ) -> Result<Json<Vec<User>>, CrakenError> {
///     Ok(Json(svc.all()))
/// }
/// ```
pub struct Inject<T: Send + Sync + 'static>(pub Arc<T>);

#[async_trait]
impl<T> FromRequestParts<Arc<Container>> for Inject<T>
where
    T: Send + Sync + 'static,
{
    type Rejection = CrakenError;

    async fn from_request_parts(
        _parts: &mut Parts,
        state: &Arc<Container>,
    ) -> Result<Self, Self::Rejection> {
        state
            .resolve::<T>()
            .map(Inject)
            .ok_or_else(|| {
                CrakenError::Internal(format!(
                    "service '{}' not registered in container",
                    std::any::type_name::<T>()
                ))
            })
    }
}
