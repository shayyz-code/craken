use axum::{
    async_trait,
    extract::{FromRequest, Request},
    Json,
};
use serde::de::DeserializeOwned;
use std::ops::Deref;
use crate::CrakenError;

pub trait Validate {
    fn validate(&self) -> Result<(), CrakenError>;
}

pub struct Validated<T>(pub T);

impl<T> Deref for Validated<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[async_trait]
impl<S, T> FromRequest<S> for Validated<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate + Send,
{
    type Rejection = CrakenError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(payload) = Json::<T>::from_request(req, state)
            .await
            .map_err(|e| CrakenError::BadRequest(e.to_string()))?;
        payload.validate()?;
        Ok(Validated(payload))
    }
}
