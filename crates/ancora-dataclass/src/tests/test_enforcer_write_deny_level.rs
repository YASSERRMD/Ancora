use crate::{ClassificationEnforcer, ClassificationPolicy, DataCategory, DataRecord, EnforcementDecision, SensitivityLevel};
#[test]
fn write_denied_when_level_exceeds_policy_max() {
    let policy = ClassificationPolicy::new("t1", SensitivityLevel::Internal);
    let record = DataRecord::new("r1", "t1", "x", SensitivityLevel::TopSecret, DataCategory::Generic, 0);
    let decision = ClassificationEnforcer::check_write(&policy, &record);
    assert!(matches!(decision, EnforcementDecision::Deny(_)));
}
