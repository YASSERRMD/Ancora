use crate::{CryptoKey, KeyAlgorithm, KeyPurpose, KeyStore};
#[test]
fn store_create_succeeds() {
    let mut store = KeyStore::new();
    let k = CryptoKey::new(
        "k1",
        "t1",
        KeyAlgorithm::Aes256,
        KeyPurpose::Encryption,
        0,
        "m",
    );
    assert!(store.create(k).is_ok());
    assert_eq!(store.total_key_ids(), 1);
}
