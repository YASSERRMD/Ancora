use crate::Severity;
#[test]
fn severity_variants_are_distinct() {
    assert_ne!(Severity::Info, Severity::Warning);
    assert_ne!(Severity::Warning, Severity::Error);
    assert_ne!(Severity::Error, Severity::Critical);
}
#[test]
fn severity_clone_equals_original() {
    let s = Severity::Critical;
    assert_eq!(s.clone(), Severity::Critical);
}
