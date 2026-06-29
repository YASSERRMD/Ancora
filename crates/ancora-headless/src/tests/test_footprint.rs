use crate::footprint::{
    check_footprint, DepRecord, FootprintManifest, FootprintMeasurement, FootprintStatus,
    FootprintTarget,
};

#[test]
fn test_within_target() {
    let m = FootprintMeasurement::new("test", 10 * 1024 * 1024, 20, 100, 200);
    let t = FootprintTarget::default();
    assert_eq!(check_footprint(&m, &t), FootprintStatus::WithinTarget);
}

#[test]
fn test_footprint_within_target_for_small_binary() {
    let m = FootprintMeasurement::new("release", 5 * 1024 * 1024, 10, 50, 100);
    let t = FootprintTarget::default();
    assert_eq!(check_footprint(&m, &t), FootprintStatus::WithinTarget);
}

#[test]
fn test_exceeds_binary_size() {
    let m = FootprintMeasurement::new("big", 100 * 1024 * 1024, 20, 100, 200);
    let t = FootprintTarget::default();
    match check_footprint(&m, &t) {
        FootprintStatus::Exceeded(v) => assert!(!v.is_empty()),
        _ => panic!("expected exceeded"),
    }
}

#[test]
fn test_binary_size_human_mb() {
    let m = FootprintMeasurement::new("x", 20 * 1024 * 1024, 5, 50, 100);
    assert!(m.binary_size_human().contains("MB"));
}

#[test]
fn test_binary_size_human_kb() {
    let m = FootprintMeasurement::new("x", 512 * 1024, 5, 50, 100);
    assert!(m.binary_size_human().contains("KB"));
}

#[test]
fn test_dep_manifest_count() {
    let mut manifest = FootprintManifest::new();
    manifest.add(DepRecord::new("serde", "1.0"));
    manifest.add(DepRecord::new("serde_json", "1.0"));
    assert_eq!(manifest.count(), 2);
}

#[test]
fn test_dep_manifest_mandatory_count() {
    let mut manifest = FootprintManifest::new();
    manifest.add(DepRecord::new("serde", "1.0"));
    manifest.add(DepRecord::new("tokio", "1.0").optional());
    assert_eq!(manifest.mandatory_count(), 1);
}
