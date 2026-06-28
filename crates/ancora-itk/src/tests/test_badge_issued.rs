use crate::badge::{issue_badge, BadgeTier};
use crate::report::{KitReport, KitStatus};

fn all_pass_report() -> KitReport {
    KitReport {
        title: "Full Pass".into(),
        status: KitStatus::AllPassed,
        lines: vec!["[PASS] kit::check - ok".into()],
        total: 4,
        passed: 4,
    }
}

fn partial_pass_report(failed: usize, total: usize) -> KitReport {
    KitReport {
        title: "Partial".into(),
        status: KitStatus::SomeFailed { failed, total },
        lines: vec![],
        total,
        passed: total - failed,
    }
}

fn empty_report() -> KitReport {
    KitReport {
        title: "Empty".into(),
        status: KitStatus::AllPassed,
        lines: vec![],
        total: 0,
        passed: 0,
    }
}

#[test]
fn badge_issued_as_compliant_on_full_pass() {
    let report = all_pass_report();
    let badge = issue_badge("my-extension", &report).unwrap();
    assert!(badge.is_compliant());
    assert_eq!(badge.tier, BadgeTier::Compliant);
    assert_eq!(badge.extension_name, "my-extension");
    assert_eq!(badge.passed, 4);
    assert_eq!(badge.total, 4);
}

#[test]
fn badge_tier_partially_compliant_when_minority_fail() {
    // 1 out of 4 fails -> minority
    let report = partial_pass_report(1, 4);
    let badge = issue_badge("ext", &report).unwrap();
    assert_eq!(badge.tier, BadgeTier::PartiallyCompliant);
    assert!(!badge.is_compliant());
}

#[test]
fn badge_tier_non_compliant_when_majority_fail() {
    // 3 out of 4 fail -> majority
    let report = partial_pass_report(3, 4);
    let badge = issue_badge("ext", &report).unwrap();
    assert_eq!(badge.tier, BadgeTier::NonCompliant);
}

#[test]
fn badge_not_issued_for_empty_report() {
    let report = empty_report();
    let result = issue_badge("ext", &report);
    assert!(result.is_none());
}

#[test]
fn badge_render_contains_extension_name_and_counts() {
    let report = all_pass_report();
    let badge = issue_badge("my-ext", &report).unwrap();
    let rendered = badge.render();
    assert!(rendered.contains("my-ext"));
    assert!(rendered.contains("4/4"));
}
