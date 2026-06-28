use crate::{DataCategory, DataRecord, DowngradePolicy, DowngradeResult, SensitivityLevel};
#[test]
fn downgrade_succeeds_to_lower_level() {
    let policy = DowngradePolicy::new(SensitivityLevel::Public);
    let mut r = DataRecord::new("r1", "t1", "x", SensitivityLevel::TopSecret, DataCategory::Generic, 0);
    let result = policy.apply(&mut r, SensitivityLevel::Internal);
    assert_eq!(result, DowngradeResult::Downgraded);
    assert_eq!(r.level, SensitivityLevel::Internal);
}
