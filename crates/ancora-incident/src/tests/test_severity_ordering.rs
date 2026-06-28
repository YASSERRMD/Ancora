use crate::incident::Severity;

#[test]
fn severity_ordering() {
    assert!(Severity::Low < Severity::Medium);
    assert!(Severity::Medium < Severity::High);
    assert!(Severity::High < Severity::Critical);
}

#[test]
fn severity_display() {
    assert_eq!(format!("{}", Severity::Low), "LOW");
    assert_eq!(format!("{}", Severity::Medium), "MEDIUM");
    assert_eq!(format!("{}", Severity::High), "HIGH");
    assert_eq!(format!("{}", Severity::Critical), "CRITICAL");
}
