use crate::summary::TenantKeySummary;
use crate::{CryptoKey, KeyAlgorithm, KeyPurpose, KeyStore};
#[test]
fn healthy_tenant_with_active_key() {
    let mut store = KeyStore::new();
    store
        .create(CryptoKey::new(
            "k1",
            "t1",
            KeyAlgorithm::Aes256,
            KeyPurpose::Encryption,
            0,
            "m",
        ))
        .unwrap();
    let summary = TenantKeySummary::generate(&store, "t1", 0, 100);
    assert_eq!(summary.active_count, 1);
    assert!(summary.is_healthy());
}
#[test]
fn empty_tenant_is_not_healthy() {
    let store = KeyStore::new();
    let summary = TenantKeySummary::generate(&store, "t2", 0, 100);
    assert_eq!(summary.active_count, 0);
    assert!(!summary.is_healthy());
}
#[test]
fn expired_key_counted_in_summary() {
    let mut store = KeyStore::new();
    store
        .create(
            CryptoKey::new(
                "k1",
                "t1",
                KeyAlgorithm::Aes256,
                KeyPurpose::Encryption,
                0,
                "m",
            )
            .with_expiry(50),
        )
        .unwrap();
    let summary = TenantKeySummary::generate(&store, "t1", 100, 10);
    assert_eq!(summary.expired_count, 1);
}
