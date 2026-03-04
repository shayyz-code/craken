use craken_http::{JwtAuth, SimpleTokenAuth, AuthenticationProvider};
use jsonwebtoken::{EncodingKey, Header, encode};

#[derive(serde::Serialize)]
struct Claims {
    sub: String,
    roles: Vec<String>,
}

#[test]
fn simple_token_authenticate() {
    let provider = SimpleTokenAuth::new("secret".to_string());
    let ok = provider.authenticate("secret").unwrap();
    assert_eq!(ok.subject, "user");
    assert!(ok.roles.is_empty());
    let err = provider.authenticate("bad").err().unwrap();
    let msg = err.to_string();
    assert!(msg.contains("invalid token"));
}

#[test]
fn jwt_authenticate() {
    let jwt = JwtAuth::new("jwtsecret".to_string());
    let claims = Claims { sub: "u1".to_string(), roles: vec!["admin".to_string()] };
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret("jwtsecret".as_bytes())).unwrap();
    let principal = jwt.authenticate(&token).unwrap();
    assert_eq!(principal.subject, "u1");
    assert_eq!(principal.roles, vec!["admin".to_string()]);
}
