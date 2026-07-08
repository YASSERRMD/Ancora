use crate::signing::{SignedCatalog, SigningKey};

fn make_key() -> SigningKey {
    SigningKey::new("test-key-1", b"super-secret-passphrase-12345".to_vec())
}

#[test]
fn unsigned_catalog_is_not_signed() {
    let catalog = SignedCatalog::new(b"catalog payload".to_vec());
    assert!(!catalog.is_signed());
}

#[test]
fn signing_marks_catalog_as_signed() {
    let mut catalog = SignedCatalog::new(b"catalog payload".to_vec());
    let key = make_key();
    catalog.sign(&key);
    assert!(catalog.is_signed());
}

#[test]
fn verify_passes_with_same_key_and_payload() {
    let payload = b"index: tool-v1.0, connector-v2.0".to_vec();
    let mut catalog = SignedCatalog::new(payload);
    let key = make_key();
    catalog.sign(&key);
    assert!(catalog.verify(&key));
}

#[test]
fn verify_fails_with_wrong_key() {
    let mut catalog = SignedCatalog::new(b"my catalog".to_vec());
    let key1 = make_key();
    let key2 = SigningKey::new("other-key", b"different-secret".to_vec());
    catalog.sign(&key1);
    assert!(!catalog.verify(&key2));
}

#[test]
fn verify_fails_after_payload_tamper() {
    let mut catalog = SignedCatalog::new(b"original payload".to_vec());
    let key = make_key();
    catalog.sign(&key);
    // Tamper with the payload.
    catalog.payload.push(b'!');
    assert!(!catalog.verify(&key));
}

#[test]
fn signature_value_is_deterministic() {
    let key = make_key();
    let sig1 = key.sign(b"hello");
    let sig2 = key.sign(b"hello");
    assert_eq!(sig1, sig2);
}

#[test]
fn different_payloads_produce_different_signatures() {
    let key = make_key();
    let sig1 = key.sign(b"payload-a");
    let sig2 = key.sign(b"payload-b");
    assert_ne!(sig1.value, sig2.value);
}
