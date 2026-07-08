use crate::key::{HsmKey, HsmKeyAlgorithm, KeyClass};
use crate::stats::HsmStats;

#[test]
fn stats_empty() {
    let s = HsmStats::from_keys(&[]);
    assert_eq!(s.total_keys, 0);
    assert_eq!(s.sensitive_ratio(), 0.0);
}

#[test]
fn stats_with_keys() {
    let k1 = HsmKey::new(1, 0, "k1", HsmKeyAlgorithm::Aes256, KeyClass::SecretKey, 1);
    let k2 = HsmKey::new(
        2,
        0,
        "k2",
        HsmKeyAlgorithm::Ed25519,
        KeyClass::PrivateKey,
        1,
    );
    let v: Vec<&HsmKey> = vec![&k1, &k2];
    let s = HsmStats::from_keys(&v);
    assert_eq!(s.total_keys, 2);
    assert_eq!(s.sensitive_count, 2);
    assert_eq!(s.sensitive_ratio(), 1.0);
}

#[test]
fn stats_by_algorithm() {
    let k1 = HsmKey::new(1, 0, "k1", HsmKeyAlgorithm::Aes256, KeyClass::SecretKey, 1);
    let k2 = HsmKey::new(2, 0, "k2", HsmKeyAlgorithm::Aes256, KeyClass::SecretKey, 1);
    let v: Vec<&HsmKey> = vec![&k1, &k2];
    let s = HsmStats::from_keys(&v);
    assert_eq!(s.by_algorithm.get("AES-256").copied(), Some(2));
}
