use crate::{ClassificationEnforcer, EnforcementDecision};
#[test]
fn is_allowed_returns_correct_booleans() {
    assert!(ClassificationEnforcer::is_allowed(&EnforcementDecision::Allow));
    assert!(!ClassificationEnforcer::is_allowed(&EnforcementDecision::Deny("no".into())));
}
