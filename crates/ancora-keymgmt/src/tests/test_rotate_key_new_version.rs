use crate::{CryptoKey, KeyAlgorithm, KeyPurpose, KeyStore, rotate_key};
#[test]
fn rotate_key_returns_new_version_number() {
    let mut store = KeyStore::new();
    store.create(CryptoKey::new("k1", "t1", KeyAlgorithm::Rsa2048, KeyPurpose::Signing, 0, "v1")).unwrap();
    let new_version = rotate_key(&mut store, "t1", "k1", "v2", 5).unwrap();
    assert_eq!(new_version, 2);
}
