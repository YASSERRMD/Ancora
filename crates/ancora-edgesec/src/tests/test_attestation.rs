use crate::attestation::{
    attest_config, attest_model, AttestationKind, AttestationRegistry, AttestationResult,
};

#[test]
fn test_model_attestation_verifies() {
    let mut reg = AttestationRegistry::new();
    let digest = vec![0xAB; 32];
    attest_model(&mut reg, "model-v1", digest.clone(), digest.clone(), 1);
    let record = reg.get("model-v1").unwrap();
    assert_eq!(record.result, AttestationResult::Valid);
    assert!(record.is_valid());
}

#[test]
fn test_model_attestation_invalid() {
    let mut reg = AttestationRegistry::new();
    let expected = vec![0xAB; 32];
    let measured = vec![0xCD; 32];
    attest_model(&mut reg, "model-v2", expected, measured, 2);
    let record = reg.get("model-v2").unwrap();
    assert_eq!(record.result, AttestationResult::Invalid);
    assert!(!record.is_valid());
}

#[test]
fn test_config_attestation_verifies() {
    let mut reg = AttestationRegistry::new();
    let digest = vec![0x42; 32];
    attest_config(&mut reg, "config-prod", digest.clone(), digest.clone(), 3);
    let record = reg.get("config-prod").unwrap();
    assert_eq!(record.result, AttestationResult::Valid);
    assert_eq!(record.kind, AttestationKind::Config);
}

#[test]
fn test_config_attestation_invalid() {
    let mut reg = AttestationRegistry::new();
    let expected = vec![0x11; 32];
    let measured = vec![0x22; 32];
    attest_config(&mut reg, "config-staging", expected, measured, 4);
    let record = reg.get("config-staging").unwrap();
    assert_eq!(record.result, AttestationResult::Invalid);
}

#[test]
fn test_registry_all_valid() {
    let mut reg = AttestationRegistry::new();
    let d = vec![0x77; 32];
    attest_model(&mut reg, "m1", d.clone(), d.clone(), 10);
    attest_config(&mut reg, "c1", d.clone(), d.clone(), 11);
    assert!(reg.all_valid());
}

#[test]
fn test_registry_invalid_artifacts() {
    let mut reg = AttestationRegistry::new();
    let expected = vec![0x01; 32];
    let measured = vec![0x02; 32];
    attest_model(&mut reg, "bad-model", expected, measured, 12);
    let bad = reg.invalid_artifacts();
    assert!(bad.contains(&"bad-model"));
}
