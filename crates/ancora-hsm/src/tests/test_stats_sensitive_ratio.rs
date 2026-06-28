use crate::key::{HsmKey, HsmKeyAlgorithm, KeyClass};
use crate::stats::HsmStats;

#[test]
fn ratio_all_sensitive() {
    let k1 = HsmKey::new(1, 0, "k1", HsmKeyAlgorithm::Aes256, KeyClass::SecretKey, 1);
    let k2 = HsmKey::new(2, 0, "k2", HsmKeyAlgorithm::Aes256, KeyClass::SecretKey, 1);
    let v: Vec<&HsmKey> = vec![&k1, &k2];
    let s = HsmStats::from_keys(&v);
    assert_eq!(s.sensitive_ratio(), 1.0);
}

#[test]
fn ratio_none_sensitive() {
    let mut k1 = HsmKey::new(1, 0, "k1", HsmKeyAlgorithm::Aes256, KeyClass::SecretKey, 1);
    let mut k2 = HsmKey::new(2, 0, "k2", HsmKeyAlgorithm::Aes256, KeyClass::SecretKey, 1);
    k1.sensitive = false;
    k2.sensitive = false;
    let v: Vec<&HsmKey> = vec![&k1, &k2];
    let s = HsmStats::from_keys(&v);
    assert_eq!(s.sensitive_ratio(), 0.0);
}

#[test]
fn ratio_half() {
    let k1 = HsmKey::new(1, 0, "k1", HsmKeyAlgorithm::Aes256, KeyClass::SecretKey, 1);
    let mut k2 = HsmKey::new(2, 0, "k2", HsmKeyAlgorithm::Aes256, KeyClass::SecretKey, 1);
    k2.sensitive = false;
    let v: Vec<&HsmKey> = vec![&k1, &k2];
    let s = HsmStats::from_keys(&v);
    assert_eq!(s.sensitive_ratio(), 0.5);
}
