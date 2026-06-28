use crate::{CryptoKey, KeyAlgorithm, KeyPurpose, KeyStore};
#[test]
fn get_active_returns_active_key() {
    let mut store = KeyStore::new();
    store.create(CryptoKey::new("k1", "t1", KeyAlgorithm::Aes256, KeyPurpose::Encryption, 0, "m")).unwrap();
    assert!(store.get_active("t1", "k1").is_ok());
    assert!(store.get_active("t1", "missing").is_err());
}
