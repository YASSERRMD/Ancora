use crate::{CryptoKey, KeyAlgorithm, KeyPurpose, KeyStore};
#[test]
fn list_tenant_active_returns_only_active() {
    let mut store = KeyStore::new();
    store.create(CryptoKey::new("k1", "t1", KeyAlgorithm::Aes256, KeyPurpose::Encryption, 0, "m")).unwrap();
    store.create(CryptoKey::new("k2", "t2", KeyAlgorithm::Ed25519, KeyPurpose::Signing, 0, "m")).unwrap();
    assert_eq!(store.list_tenant_active("t1").len(), 1);
    assert_eq!(store.list_tenant_active("t2").len(), 1);
    assert_eq!(store.list_tenant_active("t3").len(), 0);
}
