use crate::{ServiceAccount, ServiceAccountError, ServiceAccountRegistry};

#[test]
fn wrong_key_returns_invalid_key_error() {
    let mut reg = ServiceAccountRegistry::new();
    reg.register(ServiceAccount::new("svc-2", "tenant-y", "correct-hash", "desc"));
    let err = reg.authenticate("svc-2", "wrong-hash", 1000, 0).unwrap_err();
    assert_eq!(err, ServiceAccountError::InvalidKey);
}

#[test]
fn correct_key_returns_token_with_ttl() {
    let mut reg = ServiceAccountRegistry::new();
    reg.register(
        ServiceAccount::new("svc-3", "tenant-z", "hash-ok", "desc").with_scope("read:logs"),
    );
    let token = reg.authenticate("svc-3", "hash-ok", 500, 100).expect("ok");
    assert_eq!(token.expires_at_tick, 600);
    assert!(token.has_scope("read:logs"));
}
