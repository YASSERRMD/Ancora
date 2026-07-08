use crate::airgap_e2e::{AirgapBundle, AirgapRegistry, BundledPlugin};

fn build_bundle() -> AirgapBundle {
    let mut bundle = AirgapBundle::new();
    bundle
        .add(BundledPlugin::new(
            "alpha-plugin",
            "1.0.0",
            b"alpha payload".to_vec(),
        ))
        .unwrap();
    bundle
        .add(BundledPlugin::new(
            "beta-plugin",
            "2.0.0",
            b"beta payload v2".to_vec(),
        ))
        .unwrap();
    bundle
}

#[test]
fn test_airgap_bundle_install() {
    let bundle = build_bundle();
    let mut reg = AirgapRegistry::new(bundle);
    reg.install("alpha-plugin", "1.0.0")
        .expect("install must succeed");
    assert!(reg.is_installed("alpha-plugin", "1.0.0"));
}

#[test]
fn test_airgap_install_missing_fails() {
    let bundle = build_bundle();
    let mut reg = AirgapRegistry::new(bundle);
    let result = reg.install("missing", "9.9.9");
    assert!(result.is_err());
}

#[test]
fn test_airgap_checksum_integrity() {
    let plugin = BundledPlugin::new("verified", "1.0.0", b"content".to_vec());
    assert!(plugin.verify());
}

#[test]
fn test_airgap_duplicate_in_bundle_fails() {
    let mut bundle = AirgapBundle::new();
    bundle
        .add(BundledPlugin::new("dup", "1.0.0", b"v1".to_vec()))
        .unwrap();
    let result = bundle.add(BundledPlugin::new("dup", "1.0.0", b"v1".to_vec()));
    assert!(result.is_err());
}

#[test]
fn test_airgap_full_offline_workflow() {
    let bundle = build_bundle();
    assert_eq!(bundle.count(), 2);
    let mut reg = AirgapRegistry::new(bundle);
    reg.install("alpha-plugin", "1.0.0").unwrap();
    reg.install("beta-plugin", "2.0.0").unwrap();
    assert_eq!(reg.installed_count(), 2);
}
