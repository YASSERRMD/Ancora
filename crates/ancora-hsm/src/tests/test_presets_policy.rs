use crate::key::{HsmKey, HsmKeyAlgorithm, KeyClass};
use crate::presets::strict_hsm_policy;

#[test]
fn strict_policy_blocks_rsa2048() {
    let p = strict_hsm_policy();
    assert!(!p.algorithm_allowed(&HsmKeyAlgorithm::Rsa2048));
}

#[test]
fn strict_policy_allows_aes256() {
    let p = strict_hsm_policy();
    assert!(p.algorithm_allowed(&HsmKeyAlgorithm::Aes256));
}

#[test]
fn strict_policy_rejects_extractable_key() {
    let p = strict_hsm_policy();
    let mut k = HsmKey::new(1, 0, "k", HsmKeyAlgorithm::Aes256, KeyClass::SecretKey, 1);
    k.extractable = true;
    assert!(!p.is_allowed(&k));
}
