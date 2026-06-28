use crate::SensitivityLevel;
#[test]
fn numeric_matches_order() {
    assert_eq!(SensitivityLevel::Public.numeric(), 0);
    assert_eq!(SensitivityLevel::TopSecret.numeric(), 4);
}
#[test]
fn is_above_and_is_at_least() {
    let ts = SensitivityLevel::TopSecret;
    let pub_ = SensitivityLevel::Public;
    assert!(ts.is_above(&pub_));
    assert!(ts.is_at_least(&ts.clone()));
    assert!(!pub_.is_above(&ts));
}
