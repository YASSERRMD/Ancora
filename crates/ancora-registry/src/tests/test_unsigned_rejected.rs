use crate::publish::{PublishEntry, PublishError};
use crate::service::{RegistryConfig, RegistryService};
use crate::signature::{SignatureStore, TrustedKey};
use crate::versioning::Version;

fn strict_registry_with_key() -> (RegistryService, String) {
    let cfg = RegistryConfig {
        strict_signatures: true,
        ..Default::default()
    };
    let mut svc = RegistryService::new(cfg);

    let key = TrustedKey::new("test-key", vec![0xAB, 0xCD, 0xEF]);
    let version = Version::new(1, 0, 0);
    let expected_sig = SignatureStore::expected_sig(&key, "signed-tool", &version);
    svc.signatures.add_trusted_key(key);

    (svc, expected_sig)
}

#[test]
fn unsigned_entry_rejected_in_strict_mode() {
    let (mut svc, _) = strict_registry_with_key();
    let entry = PublishEntry::new(
        "signed-tool",
        Version::new(1, 0, 0),
        b"data".to_vec(),
        "alice",
    );
    let err = svc.publish(entry).unwrap_err();
    assert_eq!(err, PublishError::MissingSignature);
}

#[test]
fn entry_with_wrong_signature_rejected() {
    let (mut svc, _) = strict_registry_with_key();
    let entry = PublishEntry::new(
        "signed-tool",
        Version::new(1, 0, 0),
        b"data".to_vec(),
        "alice",
    )
    .with_signature("wrong-sig");
    let err = svc.publish(entry).unwrap_err();
    assert_eq!(err, PublishError::InvalidSignature);
}

#[test]
fn entry_with_correct_signature_accepted() {
    let (mut svc, expected_sig) = strict_registry_with_key();
    let entry = PublishEntry::new(
        "signed-tool",
        Version::new(1, 0, 0),
        b"data".to_vec(),
        "alice",
    )
    .with_signature(expected_sig);
    svc.publish(entry)
        .expect("valid signature should be accepted");
}
