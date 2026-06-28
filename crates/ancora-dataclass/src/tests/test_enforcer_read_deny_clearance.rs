use crate::{ClassificationEnforcer, ClassificationPolicy, DataCategory, DataRecord, EnforcementDecision, SensitivityLevel};
#[test]
fn read_denied_with_insufficient_clearance() {
    let policy = ClassificationPolicy::permissive("t1");
    let record = DataRecord::new("r1", "t1", "x", SensitivityLevel::TopSecret, DataCategory::Generic, 0);
    let decision = ClassificationEnforcer::check_read(&policy, &record, &SensitivityLevel::Internal);
    assert!(matches!(decision, EnforcementDecision::Deny(_)));
}
