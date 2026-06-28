use crate::builder::HsmKeyBuilder;
use crate::key::HsmKeyAlgorithm;

#[test]
fn default_not_extractable() {
    let k = HsmKeyBuilder::new(1, 0, "key", HsmKeyAlgorithm::Aes256).build();
    assert!(!k.extractable);
}

#[test]
fn explicitly_extractable() {
    let k = HsmKeyBuilder::new(1, 0, "key", HsmKeyAlgorithm::Rsa2048)
        .extractable()
        .build();
    assert!(k.extractable);
}

#[test]
fn not_sensitive_override() {
    let k = HsmKeyBuilder::new(1, 0, "key", HsmKeyAlgorithm::Aes256)
        .not_sensitive()
        .build();
    assert!(!k.sensitive);
}
