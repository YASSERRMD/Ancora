use crate::{DataCategory, DataRecord, DowngradePolicy, DowngradeResult, SensitivityLevel};
#[test]
fn downgrade_returns_already_at_or_below_when_same_level() {
    let policy = DowngradePolicy::new(SensitivityLevel::Public);
    let mut r = DataRecord::new(
        "r1",
        "t1",
        "x",
        SensitivityLevel::Internal,
        DataCategory::Generic,
        0,
    );
    let result = policy.apply(&mut r, SensitivityLevel::Internal);
    assert_eq!(result, DowngradeResult::AlreadyAtOrBelow);
}
