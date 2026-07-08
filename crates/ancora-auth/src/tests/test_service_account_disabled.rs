use crate::{ServiceAccount, ServiceAccountError, ServiceAccountRegistry};

#[test]
fn disabled_service_account_rejected() {
    let mut reg = ServiceAccountRegistry::new();
    reg.register(
        ServiceAccount::new("svc-disabled", "tenant-x", "hash-xyz", "disabled svc")
            .with_enabled(false),
    );
    let err = reg
        .authenticate("svc-disabled", "hash-xyz", 1000, 0)
        .unwrap_err();
    assert_eq!(err, ServiceAccountError::Disabled);
}

#[test]
fn disable_method_prevents_auth() {
    let mut reg = ServiceAccountRegistry::new();
    reg.register(ServiceAccount::new("svc-live", "t", "hash-live", "live"));
    let ok = reg.authenticate("svc-live", "hash-live", 100, 0);
    assert!(ok.is_ok());
    reg.disable("svc-live");
    let err = reg
        .authenticate("svc-live", "hash-live", 100, 1)
        .unwrap_err();
    assert_eq!(err, ServiceAccountError::Disabled);
}
