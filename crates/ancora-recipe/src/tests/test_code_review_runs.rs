use crate::code_review::{build, filter_by_severity, Finding, Severity};
use crate::params::ParamSet;

#[test]
fn code_review_recipe_builds() {
    let mut ps = ParamSet::new();
    ps.set("language", "rust");
    ps.set("strict", "true");
    let r = build(&ps);
    assert!(r.validate().is_ok());
    assert_eq!(r.id, "code-review");
}

#[test]
fn code_review_has_four_steps() {
    let ps = ParamSet::default();
    let r = build(&ps);
    assert_eq!(r.step_count(), 4);
}

#[test]
fn filter_keeps_warnings_and_errors() {
    let findings = vec![
        Finding::new(1, Severity::Info, "info"),
        Finding::new(2, Severity::Warning, "warn"),
        Finding::new(3, Severity::Error, "err"),
    ];
    let filtered = filter_by_severity(&findings, &Severity::Warning);
    assert_eq!(filtered.len(), 2);
}

#[test]
fn filter_with_no_matching_findings() {
    let findings = vec![Finding::new(1, Severity::Info, "info")];
    let filtered = filter_by_severity(&findings, &Severity::Error);
    assert!(filtered.is_empty());
}
