use crate::{DataCategory, DataRecord, DowngradePolicy, DowngradeResult, SensitivityLevel};
#[test]
fn downgrade_denied_when_target_below_policy_minimum() {
    let policy = DowngradePolicy::new(SensitivityLevel::Internal);
    let mut r = DataRecord::new("r1", "t1", "x", SensitivityLevel::Confidential, DataCategory::Generic, 0);
    let result = policy.apply(&mut r, SensitivityLevel::Public);
    assert!(matches!(result, DowngradeResult::Denied(_)));
}
