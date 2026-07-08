use crate::{CryptoKey, KeyAlgorithm, KeyPurpose, RotationPolicy};
#[test]
fn should_rotate_triggers_after_interval() {
    let policy = RotationPolicy::new(3).with_rotation_interval(100);
    let k = CryptoKey::new(
        "k1",
        "t1",
        KeyAlgorithm::Aes256,
        KeyPurpose::Encryption,
        0,
        "m",
    );
    assert!(!policy.should_rotate(&k, 50));
    assert!(policy.should_rotate(&k, 100));
    assert!(policy.should_rotate(&k, 200));
}
#[test]
fn policy_without_interval_never_triggers() {
    let policy = RotationPolicy::new(3);
    let k = CryptoKey::new(
        "k1",
        "t1",
        KeyAlgorithm::Aes256,
        KeyPurpose::Encryption,
        0,
        "m",
    );
    assert!(!policy.should_rotate(&k, u64::MAX));
}
