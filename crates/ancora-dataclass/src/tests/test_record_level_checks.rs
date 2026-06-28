use crate::{DataCategory, DataRecord, SensitivityLevel};
#[test]
fn level_checks_work() {
    let r = DataRecord::new("id1", "t1", "x", SensitivityLevel::Confidential, DataCategory::Generic, 0);
    assert!(r.is_above_level(&SensitivityLevel::Internal));
    assert!(r.is_at_least_level(&SensitivityLevel::Confidential));
    assert!(!r.is_above_level(&SensitivityLevel::Restricted));
}
