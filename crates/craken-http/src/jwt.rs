use crate::{AuthenticationProvider, CrakenError, Principal};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::Deserialize;

#[derive(Deserialize)]
struct Claims {
    sub: String,
    #[serde(default)]
    roles: Vec<String>,
}

pub struct JwtAuth {
    secret: String,
}

impl JwtAuth {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }
    pub fn from_env(var: &str) -> Option<Self> {
        std::env::var(var).ok().map(Self::new)
    }
}

impl AuthenticationProvider for JwtAuth {
    fn authenticate(&self, token: &str) -> Result<Principal, CrakenError> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = false;
        validation.required_spec_claims.remove("exp");
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &validation,
        )
        .map_err(|e| CrakenError::Unauthorized(e.to_string()))?;
        let c = token_data.claims;
        Ok(Principal {
            subject: c.sub,
            roles: c.roles,
        })
    }
}
