use crate::builder::{HsmKeyBuilder, SlotBuilder};
use crate::key::{HsmKeyAlgorithm, KeyClass};

#[test]
fn key_builder_defaults() {
    let k = HsmKeyBuilder::new(1, 0, "test", HsmKeyAlgorithm::Aes256).build();
    assert!(!k.extractable);
    assert!(k.sensitive);
    assert_eq!(k.class, KeyClass::SecretKey);
}

#[test]
fn key_builder_extractable() {
    let k = HsmKeyBuilder::new(1, 0, "rsa", HsmKeyAlgorithm::Rsa2048)
        .class(KeyClass::PrivateKey)
        .tick(500)
        .extractable()
        .build();
    assert!(k.extractable);
    assert_eq!(k.class, KeyClass::PrivateKey);
    assert_eq!(k.created_tick, 500);
}

#[test]
fn slot_builder() {
    let s = SlotBuilder::new(1, "HSM Slot 1").manufacturer("nShield").with_token().build();
    assert!(s.has_token());
    assert_eq!(s.manufacturer, "nShield");
    assert_eq!(s.id, 1);
}
