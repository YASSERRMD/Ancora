use crate::airgap_proof::{AirGappedAttestationBundle, AirGappedProof};

#[test]
fn test_air_gapped_attestation_via_offline_proof() {
    let device_id = "airgap-device-001";
    let boot_hash = vec![0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE, 0xBA, 0xBE];
    let tick = 42u64;
    let nonce = 12345u64;

    let proof = AirGappedProof::generate(device_id, &boot_hash, tick, nonce);
    let verified = AirGappedProof::verify(device_id, &boot_hash, tick, nonce, &proof.proof_token);
    assert!(verified, "offline proof should verify");
}

#[test]
fn test_air_gapped_proof_wrong_nonce_fails() {
    let device_id = "airgap-device-002";
    let boot_hash = vec![0x01; 8];
    let tick = 1u64;
    let nonce = 9999u64;

    let proof = AirGappedProof::generate(device_id, &boot_hash, tick, nonce);
    let verified = AirGappedProof::verify(device_id, &boot_hash, tick, nonce + 1, &proof.proof_token);
    assert!(!verified, "wrong nonce should not verify");
}

#[test]
fn test_air_gapped_proof_deterministic() {
    let device_id = "det-airgap";
    let boot_hash = vec![0xAB; 8];
    let tick = 7u64;
    let nonce = 777u64;

    let p1 = AirGappedProof::generate(device_id, &boot_hash, tick, nonce);
    let p2 = AirGappedProof::generate(device_id, &boot_hash, tick, nonce);
    assert_eq!(p1.proof_token, p2.proof_token, "proof is deterministic");
}

#[test]
fn test_air_gapped_bundle_verify_offline() {
    let device_id = "bundle-device";
    let boot_hash = vec![0x55; 16];
    let tick = 100u64;
    let nonce = 42u64;

    let proof = AirGappedProof::generate(device_id, &boot_hash, tick, nonce);
    let bundle = AirGappedAttestationBundle::new(proof, boot_hash, "model=v1 config=prod");
    assert!(bundle.verify_offline(), "bundle should verify offline");
}

#[test]
fn test_air_gapped_bundle_text_contains_device_id() {
    let device_id = "text-device";
    let boot_hash = vec![0x0F; 8];
    let proof = AirGappedProof::generate(device_id, &boot_hash, 5, 6);
    let bundle = AirGappedAttestationBundle::new(proof, boot_hash, "test");
    let text = bundle.to_text();
    assert!(text.contains("text-device"));
}
