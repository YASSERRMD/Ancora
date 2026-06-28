use crate::{CryptoKey, KeyAlgorithm, KeyPurpose, KeyStatus};
#[test]
fn destroy_clears_material_and_sets_destroyed() {
    let mut k = CryptoKey::new("k1", "t1", KeyAlgorithm::Aes256, KeyPurpose::Encryption, 0, "secret-material");
    k.destroy();
    assert_eq!(k.status, KeyStatus::Destroyed);
    assert!(k.key_material.is_empty());
}
