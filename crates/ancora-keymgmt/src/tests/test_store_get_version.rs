use crate::{CryptoKey, KeyAlgorithm, KeyPurpose, KeyStore, rotate_key};
#[test]
fn get_version_retrieves_specific_version() {
    let mut store = KeyStore::new();
    store.create(CryptoKey::new("k1", "t1", KeyAlgorithm::Aes256, KeyPurpose::Encryption, 0, "v1-material")).unwrap();
    rotate_key(&mut store, "t1", "k1", "v2-material", 10).unwrap();
    let v1 = store.get_version("t1", "k1", 1).unwrap();
    assert_eq!(v1.key_material, "v1-material");
    let v2 = store.get_version("t1", "k1", 2).unwrap();
    assert_eq!(v2.key_material, "v2-material");
}
