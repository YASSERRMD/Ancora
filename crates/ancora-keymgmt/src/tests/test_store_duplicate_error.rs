use crate::{CryptoKey, KeyAlgorithm, KeyPurpose, KeyStore};
#[test]
fn create_fails_for_duplicate_key_id() {
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
    let result = store.create(CryptoKey::new(
        "k1",
        "t1",
        KeyAlgorithm::Aes256,
        KeyPurpose::Encryption,
        1,
        "m2",
    ));
    assert!(result.is_err());
}
