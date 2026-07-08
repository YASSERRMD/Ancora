use crate::feature::FeatureState;

#[test]
fn enabled() {
    assert_eq!(FeatureState::Enabled.to_string(), "ENABLED");
}
#[test]
fn disabled() {
    assert_eq!(FeatureState::Disabled.to_string(), "DISABLED");
}
#[test]
fn beta_only() {
    assert_eq!(FeatureState::BetaOnly.to_string(), "BETA_ONLY");
}
#[test]
fn deprecated() {
    assert_eq!(FeatureState::Deprecated.to_string(), "DEPRECATED");
}
