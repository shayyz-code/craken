use crate::{CrakenError, CrakenMiddleware};
use axum::{
    extract::FromRequestParts, extract::Request, http::request::Parts, middleware::Next,
    response::IntoResponse, Router,
};
use craken_container::Container;
use std::sync::Arc;

#[derive(Clone)]
pub struct Principal {
    pub subject: String,
    pub roles: Vec<String>,
}

pub trait AuthenticationProvider: Send + Sync {
    fn authenticate(&self, token: &str) -> Result<Principal, CrakenError>;
}

pub struct AuthProvider(pub Arc<dyn AuthenticationProvider>);

pub struct AuthMiddleware {
    provider: Arc<dyn AuthenticationProvider>,
    optional: bool,
}

impl AuthMiddleware {
    pub fn new(provider: Arc<dyn AuthenticationProvider>) -> Self {
        Self {
            provider,
            optional: false,
        }
    }
    pub fn optional(provider: Arc<dyn AuthenticationProvider>) -> Self {
        Self {
            provider,
            optional: true,
        }
    }
}

impl CrakenMiddleware for AuthMiddleware {
    fn apply(&self, router: Router<Arc<Container>>) -> Router<Arc<Container>> {
        let provider = Arc::clone(&self.provider);
        let optional = self.optional;
        router.layer(axum::middleware::from_fn(move |req, next| {
            let provider = Arc::clone(&provider);
            async move { auth_fn(req, next, provider, optional).await }
        }))
    }
}

async fn auth_fn(
    mut req: Request,
    next: Next,
    provider: Arc<dyn AuthenticationProvider>,
    optional: bool,
) -> impl IntoResponse {
    let token = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "));
    match token {
        Some(t) => match provider.authenticate(t) {
            Ok(p) => {
                req.extensions_mut().insert(p);
                next.run(req).await
            }
            Err(e) => e.into_response(),
        },
        None if optional => next.run(req).await,
        None => CrakenError::Unauthorized("missing bearer token".to_string()).into_response(),
    }
}

pub struct AuthUser(pub Principal);

#[axum::async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = CrakenError;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Principal>()
            .cloned()
            .map(AuthUser)
            .ok_or_else(|| CrakenError::Unauthorized("unauthorized".to_string()))
    }
}

pub struct SimpleTokenAuth {
    expected: String,
}

impl SimpleTokenAuth {
    pub fn new(expected: String) -> Self {
        Self { expected }
    }
    pub fn from_env(var: &str) -> Option<Self> {
        std::env::var(var).ok().map(Self::new)
    }
}

impl AuthenticationProvider for SimpleTokenAuth {
    fn authenticate(&self, token: &str) -> Result<Principal, CrakenError> {
        if token == self.expected {
            Ok(Principal {
                subject: "user".to_string(),
                roles: vec![],
            })
        } else {
            Err(CrakenError::Unauthorized("invalid token".to_string()))
        }
    }
}
