use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

/// The canonical error type for the Craken framework.
///
/// Implements [`IntoResponse`] so handlers can return `Result<T, CrakenError>`
/// directly — axum will call `into_response()` automatically on the `Err` branch.
///
/// # Example
///
/// ```rust,ignore
/// async fn get_user(Path(id): Path<u64>, ctx: RequestContext) -> Result<Json<User>, CrakenError> {
///     ctx.resolve::<UserService>()
///         .ok_or_else(|| CrakenError::internal("UserService not found"))?
///         .find(id)
///         .ok_or_else(|| CrakenError::NotFound(format!("user {} not found", id)))
///         .map(Json)
/// }
/// ```
#[derive(Debug, Error)]
pub enum CrakenError {
    #[error("internal server error: {0}")]
    Internal(String),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("bad request: {0}")]
    BadRequest(String),

    #[error("unauthorized: {0}")]
    Unauthorized(String),

    #[error("forbidden: {0}")]
    Forbidden(String),

    #[error("service unavailable: {0}")]
    ServiceUnavailable(String),
}

impl CrakenError {
    /// Convenience constructor — wraps any `Display` value as an Internal error.
    pub fn internal(msg: impl ToString) -> Self {
        CrakenError::Internal(msg.to_string())
    }

    pub fn status_code(&self) -> StatusCode {
        match self {
            CrakenError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            CrakenError::NotFound(_) => StatusCode::NOT_FOUND,
            CrakenError::BadRequest(_) => StatusCode::BAD_REQUEST,
            CrakenError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            CrakenError::Forbidden(_) => StatusCode::FORBIDDEN,
            CrakenError::ServiceUnavailable(_) => StatusCode::SERVICE_UNAVAILABLE,
        }
    }

    pub fn error_code(&self) -> &'static str {
        match self {
            CrakenError::Internal(_) => "INTERNAL_ERROR",
            CrakenError::NotFound(_) => "NOT_FOUND",
            CrakenError::BadRequest(_) => "BAD_REQUEST",
            CrakenError::Unauthorized(_) => "UNAUTHORIZED",
            CrakenError::Forbidden(_) => "FORBIDDEN",
            CrakenError::ServiceUnavailable(_) => "SERVICE_UNAVAILABLE",
        }
    }
}

impl IntoResponse for CrakenError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let body = Json(json!({
            "error": {
                "code":    self.error_code(),
                "message": self.to_string(),
            }
        }));
        (status, body).into_response()
    }
}

/// Allow `?` on `anyhow::Error` inside handlers that return `CrakenError`.
impl From<anyhow::Error> for CrakenError {
    fn from(err: anyhow::Error) -> Self {
        CrakenError::Internal(err.to_string())
    }
}
