use crate::classifier::SafetyClassifier;
use crate::pii::{PiiDetector, PiiKind};

#[test]
fn email_in_output_is_flagged() {
    let d = PiiDetector::new();
    let result = d.detect("Please contact john.doe@company.org for details.");
    assert!(result.is_some());
    assert_eq!(result.unwrap().kind, PiiKind::Email);
}

#[test]
fn ssn_in_output_is_flagged() {
    let d = PiiDetector::new();
    let result = d.detect("The patient SSN is 987-65-4321.");
    assert!(result.is_some());
    assert_eq!(result.unwrap().kind, PiiKind::SocialSecurityNumber);
}

#[test]
fn phone_number_in_output_is_flagged() {
    let d = PiiDetector::new();
    let result = d.detect("Call us at 555-867-5309 anytime.");
    assert!(result.is_some());
    assert_eq!(result.unwrap().kind, PiiKind::PhoneNumber);
}

#[test]
fn classifier_reports_pii_detected() {
    let clf = SafetyClassifier::new();
    let report = clf.classify("The user email is admin@internal.example.com.");
    assert!(report.pii_detected);
}

#[test]
fn no_pii_not_flagged() {
    let d = PiiDetector::new();
    let result = d.detect("The answer is 42 and the sum is 100.");
    assert!(result.is_none());
}
