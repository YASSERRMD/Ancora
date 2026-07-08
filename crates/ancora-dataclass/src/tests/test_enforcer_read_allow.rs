use crate::{
    ClassificationEnforcer, ClassificationPolicy, DataCategory, DataRecord, EnforcementDecision,
    SensitivityLevel,
};
#[test]
fn read_allowed_with_sufficient_clearance() {
    let policy = ClassificationPolicy::permissive("t1");
    let record = DataRecord::new(
        "r1",
        "t1",
        "x",
        SensitivityLevel::Confidential,
        DataCategory::Generic,
        0,
    );
    let decision =
        ClassificationEnforcer::check_read(&policy, &record, &SensitivityLevel::TopSecret);
    assert_eq!(decision, EnforcementDecision::Allow);
}
