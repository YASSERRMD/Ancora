use crate::{ServiceAccount, ServiceAccountError, ServiceAccountRegistry, TokenKind};

fn make_registry() -> ServiceAccountRegistry {
    let mut reg = ServiceAccountRegistry::new();
    reg.register(
        ServiceAccount::new("svc-1", "tenant-x", "hash-abc", "CI pipeline")
            .with_scope("read:agents")
            .with_scope("write:tasks"),
    );
    reg
}

#[test]
fn service_account_auth_success() {
    let reg = make_registry();
    let token = reg.authenticate("svc-1", "hash-abc", 1000, 0).expect("auth ok");
    assert_eq!(token.subject, "svc-1");
    assert_eq!(token.tenant_id, "tenant-x");
    assert!(matches!(token.kind, TokenKind::ServiceAccount));
    assert!(token.has_scope("read:agents"));
}

#[test]
fn service_account_wrong_key_rejected() {
    let reg = make_registry();
    let err = reg.authenticate("svc-1", "wrong-hash", 1000, 0).unwrap_err();
    assert_eq!(err, ServiceAccountError::InvalidKey);
}

#[test]
fn service_account_not_found_rejected() {
    let reg = make_registry();
    let err = reg.authenticate("unknown", "hash-abc", 1000, 0).unwrap_err();
    assert_eq!(err, ServiceAccountError::NotFound);
}
