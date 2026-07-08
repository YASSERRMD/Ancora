use crate::{DataCategory, DataClassStats, DataRecord, SensitivityLevel};
#[test]
fn stats_counts_records_by_level() {
    let records = [
        DataRecord::new(
            "r1",
            "t1",
            "x",
            SensitivityLevel::Internal,
            DataCategory::Generic,
            0,
        ),
        DataRecord::new(
            "r2",
            "t1",
            "y",
            SensitivityLevel::Internal,
            DataCategory::Generic,
            0,
        ),
        DataRecord::new(
            "r3",
            "t1",
            "z",
            SensitivityLevel::TopSecret,
            DataCategory::Generic,
            0,
        ),
    ];
    let stats = DataClassStats::from_records(records.iter());
    assert_eq!(stats.total, 3);
    assert_eq!(stats.highest_level, Some(SensitivityLevel::TopSecret));
}
