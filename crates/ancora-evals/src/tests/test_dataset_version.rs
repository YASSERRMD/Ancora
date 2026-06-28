use crate::dataset::Dataset;

#[test]
fn test_version_is_stored() {
    let dataset = Dataset::new("v-test", "2.3.1");
    assert_eq!(dataset.version, "2.3.1");
    assert_eq!(dataset.name, "v-test");
}

#[test]
fn test_version_can_be_any_string() {
    let dataset = Dataset::new("d", "nightly-2026-06-28");
    assert_eq!(dataset.version, "nightly-2026-06-28");
}

#[test]
fn test_datasets_can_have_same_name_different_version() {
    let d1 = Dataset::new("benchmark", "1.0.0");
    let d2 = Dataset::new("benchmark", "2.0.0");
    assert_ne!(d1.version, d2.version);
    assert_eq!(d1.name, d2.name);
}
