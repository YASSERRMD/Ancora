use crate::{JwkKey, JwksStore, JwtClaims, JwtError, JwtValidator};

fn make_store_with_key() -> JwksStore {
    let mut store = JwksStore::new();
    store.add_key(JwkKey::new("k1", "modulus", "AQAB", 0, 1000));
    store
}

#[test]
fn jwt_valid_claims_return_token() {
    let store = make_store_with_key();
    let validator = JwtValidator::new(&store, "iss.example.com", "aud.example.com");
    let claims = JwtClaims::new("user1", "iss.example.com", "aud.example.com", "tenant-x", 0, 500);
    let token = validator.validate("k1", &claims, 100).expect("valid");
    assert_eq!(token.subject, "user1");
}

#[test]
fn jwt_unknown_kid_rejected() {
    let store = make_store_with_key();
    let validator = JwtValidator::new(&store, "iss.example.com", "aud.example.com");
    let claims = JwtClaims::new("user1", "iss.example.com", "aud.example.com", "tenant-x", 0, 500);
    let err = validator.validate("unknown-kid", &claims, 100).unwrap_err();
    assert!(matches!(err, JwtError::UnknownKid(_)));
}

#[test]
fn jwt_expired_claims_rejected() {
    let store = make_store_with_key();
    let validator = JwtValidator::new(&store, "iss.example.com", "aud.example.com");
    let claims = JwtClaims::new("user1", "iss.example.com", "aud.example.com", "tenant-x", 0, 50);
    let err = validator.validate("k1", &claims, 100).unwrap_err();
    assert_eq!(err, JwtError::TokenExpired);
}

#[test]
fn jwt_wrong_issuer_rejected() {
    let store = make_store_with_key();
    let validator = JwtValidator::new(&store, "iss.example.com", "aud.example.com");
    let claims = JwtClaims::new("user1", "other-iss.com", "aud.example.com", "tenant-x", 0, 500);
    let err = validator.validate("k1", &claims, 100).unwrap_err();
    assert!(matches!(err, JwtError::InvalidClaims(_)));
}
