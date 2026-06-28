use crate::feature::{FeatureFlag, FeatureRegistry, FeatureState};

#[test]
fn empty_registry() {
    let r = FeatureRegistry::new();
    assert_eq!(r.count(), 0);
    assert_eq!(r.enabled_count(), 0);
    assert!(!r.is_enabled("anything"));
}

#[test]
fn register_and_query() {
    let mut r = FeatureRegistry::new();
    r.register(FeatureFlag::new("hsm", FeatureState::Enabled, "d"));
    r.register(FeatureFlag::new("beta", FeatureState::BetaOnly, "d"));
    assert_eq!(r.count(), 2);
    assert_eq!(r.enabled_count(), 1);
    assert!(r.is_enabled("hsm"));
    assert!(!r.is_enabled("beta"));
    assert!(!r.is_enabled("missing"));
}

#[test]
fn enable_disable() {
    let mut r = FeatureRegistry::new();
    r.register(FeatureFlag::new("feat", FeatureState::Disabled, "d"));
    assert!(!r.is_enabled("feat"));
    r.enable("feat");
    assert!(r.is_enabled("feat"));
    r.disable("feat");
    assert!(!r.is_enabled("feat"));
}

#[test]
fn all_iterator() {
    let mut r = FeatureRegistry::new();
    r.register(FeatureFlag::new("f1", FeatureState::Enabled, "d"));
    assert_eq!(r.all().count(), 1);
}
