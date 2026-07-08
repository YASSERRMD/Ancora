use crate::{
    ClassificationEnforcer, ClassificationPolicy, DataCategory, DataRecord, EnforcementDecision,
    SensitivityLevel,
};
#[test]
fn write_allowed_when_level_within_policy() {
    let policy = ClassificationPolicy::permissive("t1");
    let record = DataRecord::new(
        "r1",
        "t1",
        "x",
        SensitivityLevel::Confidential,
        DataCategory::Generic,
        0,
    );
    assert_eq!(
        ClassificationEnforcer::check_write(&policy, &record),
        EnforcementDecision::Allow
    );
}
