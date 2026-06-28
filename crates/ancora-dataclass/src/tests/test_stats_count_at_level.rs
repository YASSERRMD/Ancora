use crate::{DataCategory, DataClassStats, DataRecord, SensitivityLevel};
#[test]
fn count_at_level_returns_correct_count() {
    let records = vec![
        DataRecord::new("r1", "t1", "x", SensitivityLevel::Confidential, DataCategory::Generic, 0),
        DataRecord::new("r2", "t1", "y", SensitivityLevel::Confidential, DataCategory::Generic, 0),
        DataRecord::new("r3", "t1", "z", SensitivityLevel::Public, DataCategory::Generic, 0),
    ];
    let stats = DataClassStats::from_records(records.iter());
    assert_eq!(stats.count_at_level(&SensitivityLevel::Confidential), 2);
    assert_eq!(stats.count_at_level(&SensitivityLevel::TopSecret), 0);
}
