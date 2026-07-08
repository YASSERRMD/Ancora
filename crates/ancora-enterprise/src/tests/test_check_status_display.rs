use crate::checkpoint::CheckStatus;

#[test]
fn pass() {
    assert_eq!(CheckStatus::Pass.to_string(), "PASS");
}
#[test]
fn warn() {
    assert_eq!(CheckStatus::Warn.to_string(), "WARN");
}
#[test]
fn fail() {
    assert_eq!(CheckStatus::Fail.to_string(), "FAIL");
}
#[test]
fn not_applicable() {
    assert_eq!(CheckStatus::NotApplicable.to_string(), "NOT_APPLICABLE");
}
