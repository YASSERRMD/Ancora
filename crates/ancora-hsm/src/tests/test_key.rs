use crate::key::{HsmKey, HsmKeyAlgorithm, KeyClass};

#[test]
fn key_defaults() {
    let k = HsmKey::new(1, 0, "aes-key", HsmKeyAlgorithm::Aes256, KeyClass::SecretKey, 100);
    assert!(!k.extractable);
    assert!(k.sensitive);
    assert_eq!(k.handle, 1);
    assert_eq!(k.slot_id, 0);
}

#[test]
fn key_with_attribute() {
    let k = HsmKey::new(1, 0, "key", HsmKeyAlgorithm::Ed25519, KeyClass::PrivateKey, 1)
        .with_attribute("purpose", "signing");
    assert_eq!(k.attributes.get("purpose").map(|s| s.as_str()), Some("signing"));
}
