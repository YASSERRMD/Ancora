use crate::key::{HsmKey, HsmKeyAlgorithm, KeyClass};
use crate::policy::HsmPolicy;

#[test]
fn policy_default_no_extractable() {
    let k = HsmKey::new(1, 0, "k", HsmKeyAlgorithm::Aes256, KeyClass::SecretKey, 1);
    let policy = HsmPolicy::new();
    assert!(policy.is_allowed(&k));
}

#[test]
fn policy_block_extractable() {
    let mut k = HsmKey::new(1, 0, "k", HsmKeyAlgorithm::Aes256, KeyClass::SecretKey, 1);
    k.extractable = true;
    let policy = HsmPolicy::new();
    assert!(!policy.is_allowed(&k));
}

#[test]
fn policy_blocked_algorithm() {
    let k = HsmKey::new(1, 0, "k", HsmKeyAlgorithm::Rsa2048, KeyClass::PrivateKey, 1);
    let policy = HsmPolicy::new().block_algorithm(HsmKeyAlgorithm::Rsa2048);
    assert!(!policy.is_allowed(&k));
    assert!(!policy.algorithm_allowed(&HsmKeyAlgorithm::Rsa2048));
}

#[test]
fn policy_allowed_algorithm() {
    let policy = HsmPolicy::new().block_algorithm(HsmKeyAlgorithm::Rsa2048);
    assert!(policy.algorithm_allowed(&HsmKeyAlgorithm::Aes256));
}
