use crate::trust_e2e::{PluginManifest, TrustGate, TrustLevel, TrustPolicy};

#[test]
fn test_trust_policy_blocks_low_trust_install() {
    let policy = TrustPolicy::new(TrustLevel::Verified);
    let gate = TrustGate::new(policy);
    let manifest = PluginManifest::new(
        "untrusted-plugin",
        TrustLevel::Community,
        false,
        Some("abc123"),
    );
    let result = gate.check(&manifest);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("below required"));
}

#[test]
fn test_trust_policy_allows_high_trust() {
    let policy = TrustPolicy::new(TrustLevel::Verified);
    let gate = TrustGate::new(policy);
    let manifest = PluginManifest::new(
        "verified-plugin",
        TrustLevel::Official,
        true,
        Some("checksum"),
    );
    gate.check(&manifest).expect("official plugin must pass");
}

#[test]
fn test_trust_gate_requires_checksum() {
    let policy = TrustPolicy::new(TrustLevel::Community);
    let gate = TrustGate::new(policy);
    let manifest = PluginManifest::new("no-checksum", TrustLevel::Verified, true, None);
    let result = gate.check(&manifest);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("checksum"));
}

#[test]
fn test_trust_gate_blocks_unverified_publisher() {
    let policy = TrustPolicy::new(TrustLevel::Community);
    let gate = TrustGate::new(policy);
    let manifest = PluginManifest::new("unverified-pub", TrustLevel::Verified, false, Some("sum"));
    let result = gate.check(&manifest);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("publisher"));
}

#[test]
fn test_strict_policy_only_accepts_official() {
    let gate = TrustGate::new(TrustPolicy::strict());
    let verified = PluginManifest::new("verified", TrustLevel::Verified, true, Some("sum"));
    let official = PluginManifest::new("official", TrustLevel::Official, true, Some("sum"));
    assert!(gate.check(&verified).is_err());
    gate.check(&official)
        .expect("official must pass strict policy");
}

#[test]
fn test_permissive_policy_accepts_community() {
    let gate = TrustGate::new(TrustPolicy::permissive());
    let manifest = PluginManifest::new("comm-plugin", TrustLevel::Community, false, None);
    gate.check(&manifest)
        .expect("community must pass permissive policy");
}
