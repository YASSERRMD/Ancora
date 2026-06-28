use crate::breaking_detector::{ci_check_stability, detect_breaking_changes, ApiSnapshot};
use crate::semver::SemVer;
use crate::stability_policy::StabilityLevel;

#[test]
fn breaking_change_flagged_in_ci_for_stable_api() {
    let old = ApiSnapshot::new(
        SemVer::new(1, 0, 0),
        vec!["on_load", "on_unload", "on_message"],
    );
    let new = ApiSnapshot::new(
        SemVer::new(2, 0, 0),
        vec!["on_load", "on_unload"], // on_message removed
    );
    // Stable API with 0 deprecation cycles should fail.
    let result = ci_check_stability(&old, &new, &StabilityLevel::Stable, 0);
    assert!(result.is_err(), "expected CI to flag the breaking change");
    let changes = result.unwrap_err();
    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0].item, "on_message");
}

#[test]
fn no_breaking_change_passes_ci() {
    let old = ApiSnapshot::new(SemVer::new(1, 0, 0), vec!["foo", "bar"]);
    let new = ApiSnapshot::new(SemVer::new(1, 1, 0), vec!["foo", "bar"]);
    let result = ci_check_stability(&old, &new, &StabilityLevel::Stable, 0);
    assert!(result.is_ok());
}

#[test]
fn unstable_api_allows_breaking_change_immediately() {
    let old = ApiSnapshot::new(SemVer::new(0, 1, 0), vec!["alpha_hook"]);
    let new = ApiSnapshot::new(SemVer::new(0, 2, 0), Vec::<&str>::new());
    let result = ci_check_stability(&old, &new, &StabilityLevel::Unstable, 0);
    assert!(result.is_ok(), "unstable API may break without notice");
}

#[test]
fn detect_multiple_removed_endpoints() {
    let old = ApiSnapshot::new(SemVer::new(1, 0, 0), vec!["a", "b", "c"]);
    let new = ApiSnapshot::new(SemVer::new(2, 0, 0), vec!["a"]);
    let changes = detect_breaking_changes(&old, &new);
    assert_eq!(changes.len(), 2);
}
