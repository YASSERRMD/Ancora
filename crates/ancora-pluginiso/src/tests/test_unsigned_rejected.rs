use crate::signature::{PluginSignature, SignatureError, SignaturePolicy, SignatureVerifier, TrustedKey};

fn make_verifier_with_key(key_id: &str, key_bytes: Vec<u8>) -> SignatureVerifier {
    let mut v = SignatureVerifier::new();
    v.add_trusted_key(TrustedKey { key_id: key_id.into(), public_key_bytes: key_bytes });
    v
}

#[test]
fn unsigned_plugin_rejected_in_strict_mode() {
    let v = make_verifier_with_key("k1", vec![0xAA, 0xBB]);
    let err = v
        .verify(b"plugin content", None, &SignaturePolicy::Required)
        .unwrap_err();
    assert_eq!(err, SignatureError::MissingSignature);
}

#[test]
fn unsigned_plugin_allowed_in_optional_mode() {
    let v = make_verifier_with_key("k1", vec![0xAA, 0xBB]);
    assert!(v.verify(b"plugin content", None, &SignaturePolicy::Optional).is_ok());
}

#[test]
fn unsigned_plugin_allowed_in_disabled_mode() {
    let v = SignatureVerifier::new(); // no trusted keys
    assert!(v.verify(b"plugin content", None, &SignaturePolicy::Disabled).is_ok());
}

#[test]
fn valid_signature_accepted_in_strict_mode() {
    let key_bytes = vec![0xDE, 0xAD, 0xBE, 0xEF];
    let v = make_verifier_with_key("k2", key_bytes.clone());
    let content = b"trusted plugin bytes";
    let sig_bytes = SignatureVerifier::stub_sign(&key_bytes, content);
    let sig = PluginSignature { key_id: "k2".into(), signature_bytes: sig_bytes };

    assert!(v.verify(content, Some(&sig), &SignaturePolicy::Required).is_ok());
}

#[test]
fn tampered_content_rejected() {
    let key_bytes = vec![0xCA, 0xFE];
    let v = make_verifier_with_key("k3", key_bytes.clone());
    let original_content = b"original bytes";
    let sig_bytes = SignatureVerifier::stub_sign(&key_bytes, original_content);
    let sig = PluginSignature { key_id: "k3".into(), signature_bytes: sig_bytes };

    // Verify against tampered content.
    let err = v
        .verify(b"tampered bytes", Some(&sig), &SignaturePolicy::Required)
        .unwrap_err();
    assert_eq!(err, SignatureError::InvalidSignature);
}

#[test]
fn unknown_key_id_rejected() {
    let v = make_verifier_with_key("known-key", vec![1, 2, 3]);
    let sig = PluginSignature { key_id: "unknown-key".into(), signature_bytes: vec![0] };
    let err = v
        .verify(b"content", Some(&sig), &SignaturePolicy::Required)
        .unwrap_err();
    assert!(matches!(err, SignatureError::UnknownKey(_)));
}
