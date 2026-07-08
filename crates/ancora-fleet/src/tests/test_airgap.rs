use crate::airgap::*;
use crate::registration::DeviceId;

#[test]
fn test_airgapped_fleet_via_offline_bundles() {
    let mut manager = AirGapFleetManager::new();
    let mut bundle = OfflineBundle::new("bundle-v1", "firmware and models");
    bundle.add_file("firmware.bin", vec![0x01, 0x02, 0x03, 0xDE, 0xAD]);
    bundle.add_file("model-tiny.bin", vec![0xFF; 128]);

    assert!(bundle.verify());
    assert_eq!(bundle.file_count(), 2);

    manager.add_bundle(bundle);
    assert_eq!(manager.bundle_count(), 1);

    let ids: Vec<DeviceId> = (0..3)
        .map(|i| DeviceId::new(format!("dev-{}", i)))
        .collect();
    let records = manager.apply_to_fleet(&ids, "bundle-v1");

    assert_eq!(records.len(), 3);
    for r in &records {
        assert_eq!(r.status, BundleApplyStatus::Applied);
    }
}

#[test]
fn test_bundle_verification_fails_on_corrupt_data() {
    let mut bundle = OfflineBundle::new("bundle-corrupt", "test");
    bundle.add_file("data.bin", vec![1, 2, 3]);

    // Manually corrupt the manifest checksum
    bundle
        .manifest
        .insert("data.bin".into(), "bad-checksum".into());
    assert!(!bundle.verify());
}

#[test]
fn test_apply_missing_bundle_returns_error() {
    let mut manager = AirGapFleetManager::new();
    let id = DeviceId::new("dev-001");
    let record = manager.apply_bundle(&id, "nonexistent-bundle");
    assert!(matches!(record.status, BundleApplyStatus::Error(_)));
}
