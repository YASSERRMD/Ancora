use crate::{JwkKey, JwksStore, JwtClaims, JwtError, JwtValidator};

#[test]
fn validator_uses_new_key_after_rotation() {
    let mut store = JwksStore::new();
    store.add_key(JwkKey::new("old-key", "mod-old", "AQAB", 0, 500));
    {
        let validator = JwtValidator::new(&store, "iss.test", "aud.test");
        let claims = JwtClaims::new("u", "iss.test", "aud.test", "t", 0, 400);
        assert!(validator.validate("old-key", &claims, 100).is_ok());
    }
    let new_key = JwkKey::new("new-key", "mod-new", "AQAB", 500, 9999);
    store.rotate("old-key", new_key);
    {
        let validator = JwtValidator::new(&store, "iss.test", "aud.test");
        let claims = JwtClaims::new("u", "iss.test", "aud.test", "t", 0, 2000);
        let err = validator.validate("old-key", &claims, 600).unwrap_err();
        assert!(matches!(err, JwtError::UnknownKid(_)));
        assert!(validator.validate("new-key", &claims, 600).is_ok());
    }
}

#[test]
fn validator_rejects_key_outside_validity_window() {
    let mut store = JwksStore::new();
    store.add_key(JwkKey::new("k1", "mod", "AQAB", 100, 200));
    let validator = JwtValidator::new(&store, "iss.test", "aud.test");
    let claims = JwtClaims::new("u", "iss.test", "aud.test", "t", 0, 500);
    let err = validator.validate("k1", &claims, 50).unwrap_err();
    assert_eq!(err, JwtError::KeyExpired);
    let err = validator.validate("k1", &claims, 250).unwrap_err();
    assert_eq!(err, JwtError::KeyExpired);
    assert!(validator.validate("k1", &claims, 150).is_ok());
}
