use crate::{CryptoKey, KeyAlgorithm, KeyPurpose};
#[test]
fn with_metadata_stores_key_value() {
    let k = CryptoKey::new("k1", "t1", KeyAlgorithm::Aes256, KeyPurpose::Encryption, 0, "m")
        .with_metadata("label", "production");
    assert_eq!(k.metadata.get("label").map(String::as_str), Some("production"));
}
