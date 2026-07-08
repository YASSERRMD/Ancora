use crate::{CryptoKey, KeyAlgorithm, KeyPurpose, KeyStats, KeyStore};
#[test]
fn stats_for_tenant_counts_active_keys() {
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
    store
        .create(CryptoKey::new(
            "k2",
            "t1",
            KeyAlgorithm::Ed25519,
            KeyPurpose::Signing,
            0,
            "m",
        ))
        .unwrap();
    let stats = KeyStats::for_tenant(&store, "t1");
    assert_eq!(stats.total_active, 2);
    assert!(stats.by_algorithm.contains_key("AES-256"));
}
