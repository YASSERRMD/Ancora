use crate::{DataCategory, DataRecord, DataRegistry, SensitivityLevel};
#[test]
fn at_or_above_filters_by_level() {
    let mut reg = DataRegistry::new();
    reg.insert(DataRecord::new(
        "r1",
        "t1",
        "x",
        SensitivityLevel::Public,
        DataCategory::Generic,
        0,
    ))
    .unwrap();
    reg.insert(DataRecord::new(
        "r2",
        "t1",
        "y",
        SensitivityLevel::TopSecret,
        DataCategory::Generic,
        0,
    ))
    .unwrap();
    let high = reg.at_or_above(&SensitivityLevel::Confidential);
    assert_eq!(high.len(), 1);
    assert_eq!(high[0].id, "r2");
}
