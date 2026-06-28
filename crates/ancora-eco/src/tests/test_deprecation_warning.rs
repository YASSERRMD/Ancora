use crate::deprecation::{check_deprecation, DeprecationMarker};
use crate::semver::SemVer;

#[test]
fn deprecation_warning_emitted_in_window() {
    let marker = DeprecationMarker::new(
        SemVer::new(1, 3, 0),
        SemVer::new(2, 0, 0),
        "Use new_hook instead",
    );
    let current = SemVer::new(1, 5, 0);
    let warning = check_deprecation("legacy_hook", &marker, &current);
    assert!(warning.is_some(), "expected a deprecation warning");
    let w = warning.unwrap();
    let msg = w.format();
    assert!(msg.contains("legacy_hook"));
    assert!(msg.contains("1.3.0"));
    assert!(msg.contains("2.0.0"));
}

#[test]
fn no_deprecation_warning_before_since_version() {
    let marker = DeprecationMarker::new(
        SemVer::new(1, 8, 0),
        SemVer::new(2, 0, 0),
        "Use new_hook instead",
    );
    let current = SemVer::new(1, 4, 0);
    assert!(
        check_deprecation("some_hook", &marker, &current).is_none(),
        "expected no warning before deprecation window"
    );
}

#[test]
fn item_is_removed_after_removed_in_version() {
    let marker = DeprecationMarker::new(
        SemVer::new(1, 0, 0),
        SemVer::new(2, 0, 0),
        "Removed",
    );
    assert!(marker.is_removed_at(&SemVer::new(2, 0, 0)));
    assert!(!marker.is_removed_at(&SemVer::new(1, 9, 0)));
}
