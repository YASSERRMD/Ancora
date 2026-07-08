use crate::classifier::SafetyClassifier;
use crate::policy_violation::{PolicyViolationDetector, ViolationKind};

#[test]
fn confidential_keyword_triggers_policy() {
    let d = PolicyViolationDetector::new();
    let v = d.check("This report contains confidential figures.");
    assert!(v.is_some());
    assert_eq!(v.unwrap().kind, ViolationKind::ConfidentialDataExposure);
}

#[test]
fn security_bypass_triggers_policy() {
    let d = PolicyViolationDetector::new();
    let v = d.check("Step 3: bypass security validation entirely.");
    assert!(v.is_some());
    assert_eq!(v.unwrap().kind, ViolationKind::RestrictedInstruction);
}

#[test]
fn clean_text_passes_policy() {
    let d = PolicyViolationDetector::new();
    let v = d.check("Please review the attached proposal.");
    assert!(v.is_none());
}

#[test]
fn classifier_reports_policy_violation() {
    let clf = SafetyClassifier::new();
    let report = clf.classify("Do not share these internal only metrics.");
    assert!(report.policy_violation);
}

#[test]
fn multiple_policy_rules_all_detected() {
    let d = PolicyViolationDetector::new();
    let vs = d.check_all("confidential bypass security trade secret");
    assert!(vs.len() >= 2);
}
