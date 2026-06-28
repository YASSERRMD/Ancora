use crate::{KeyAlgorithm, KeyBuilder, KeyPurpose, KeyStatus};
#[test]
fn builder_defaults_to_aes256_encryption() {
    let k = KeyBuilder::new("k1", "t1").build();
    assert_eq!(k.algorithm, KeyAlgorithm::Aes256);
    assert_eq!(k.purpose, KeyPurpose::Encryption);
    assert_eq!(k.status, KeyStatus::Active);
    assert_eq!(k.version, 1);
}
