//! Tests: incompatible plugins are rejected by the compatibility checker.

use crate::compatibility::{check_compatibility, CompatError};
use crate::manifest::{ManifestBuilder, PluginKind, SemVer};

fn make_manifest(min: SemVer, max: SemVer) -> crate::manifest::PluginManifest {
    ManifestBuilder::new()
        .id("compat-test-plugin")
        .name("Compat Test Plugin")
        .version(SemVer::new(1, 0, 0))
        .sdk_range(min, max)
        .kind(PluginKind::Tool)
        .build()
        .unwrap()
}

#[test]
fn plugin_compatible_with_exact_min() {
    let m = make_manifest(SemVer::new(1, 0, 0), SemVer::new(2, 0, 0));
    assert!(check_compatibility(&m, &SemVer::new(1, 0, 0)).is_ok());
}

#[test]
fn plugin_compatible_with_exact_max() {
    let m = make_manifest(SemVer::new(1, 0, 0), SemVer::new(2, 0, 0));
    assert!(check_compatibility(&m, &SemVer::new(2, 0, 0)).is_ok());
}

#[test]
fn plugin_compatible_within_range() {
    let m = make_manifest(SemVer::new(1, 0, 0), SemVer::new(3, 0, 0));
    assert!(check_compatibility(&m, &SemVer::new(2, 5, 0)).is_ok());
}

#[test]
fn sdk_below_min_is_rejected() {
    let m = make_manifest(SemVer::new(2, 0, 0), SemVer::new(3, 0, 0));
    let result = check_compatibility(&m, &SemVer::new(1, 9, 9));
    assert!(matches!(result, Err(CompatError::IncompatibleSdk { .. })));
}

#[test]
fn sdk_above_max_is_rejected() {
    let m = make_manifest(SemVer::new(1, 0, 0), SemVer::new(1, 99, 0));
    let result = check_compatibility(&m, &SemVer::new(2, 0, 0));
    assert!(matches!(result, Err(CompatError::IncompatibleSdk { .. })));
}

#[test]
fn error_contains_version_information() {
    let m = make_manifest(SemVer::new(2, 0, 0), SemVer::new(3, 0, 0));
    let err = check_compatibility(&m, &SemVer::new(1, 0, 0)).unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("1.0.0"), "error should mention the sdk version: {msg}");
}
