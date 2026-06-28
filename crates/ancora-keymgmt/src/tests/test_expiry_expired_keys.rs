use crate::{CryptoKey, ExpiryChecker, KeyAlgorithm, KeyPurpose, KeyStore};
#[test]
fn expired_keys_returns_keys_past_expiry() {
    let mut store = KeyStore::new();
    store.create(CryptoKey::new("k1", "t1", KeyAlgorithm::Aes256, KeyPurpose::Encryption, 0, "m").with_expiry(50)).unwrap();
    store.create(CryptoKey::new("k2", "t1", KeyAlgorithm::Aes256, KeyPurpose::Encryption, 0, "m").with_expiry(200)).unwrap();
    let expired = ExpiryChecker::expired_keys(&store, "t1", 100);
    assert_eq!(expired.len(), 1);
    assert_eq!(expired[0].id, "k1");
}
