use crate::feature::{FeatureFlag, FeatureState};

#[test]
fn enabled_is_active() {
    let f = FeatureFlag::new("hsm", FeatureState::Enabled, "HSM integration");
    assert!(f.is_active());
    assert!(!f.is_beta());
}

#[test]
fn beta_is_not_active() {
    let f = FeatureFlag::new("beta-feat", FeatureState::BetaOnly, "Beta feature");
    assert!(!f.is_active());
    assert!(f.is_beta());
}

#[test]
fn disabled_not_active() {
    let f = FeatureFlag::new("feat", FeatureState::Disabled, "desc");
    assert!(!f.is_active());
    assert!(!f.is_beta());
}
