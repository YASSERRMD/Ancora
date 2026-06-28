use crate::{ClassificationEnforcer, ClassificationPolicy, DataCategory, DataRecord, EnforcementDecision, SensitivityLevel};
#[test]
fn write_denied_when_strict_policy_requires_tag_but_none_present() {
    let policy = ClassificationPolicy::strict("t1");
    let record = DataRecord::new("r1", "t1", "x", SensitivityLevel::Internal, DataCategory::Generic, 0);
    let decision = ClassificationEnforcer::check_write(&policy, &record);
    assert!(matches!(decision, EnforcementDecision::Deny(_)));
}
#[test]
fn write_allowed_when_tag_provided() {
    let policy = ClassificationPolicy::strict("t1");
    let record = DataRecord::new("r1", "t1", "x", SensitivityLevel::Internal, DataCategory::Generic, 0)
        .with_tag("gdpr");
    assert_eq!(ClassificationEnforcer::check_write(&policy, &record), EnforcementDecision::Allow);
}
