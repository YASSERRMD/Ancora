use crate::{CryptoKey, KeyAlgorithm, KeyPurpose, KeyStore, rotate_key};
#[test]
fn version_count_increments_after_rotation() {
    let mut store = KeyStore::new();
    store.create(CryptoKey::new("k1", "t1", KeyAlgorithm::Aes256, KeyPurpose::Encryption, 0, "m1")).unwrap();
    rotate_key(&mut store, "t1", "k1", "m2", 10).unwrap();
    assert_eq!(store.version_count("t1", "k1"), 2);
}
