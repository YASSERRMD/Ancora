use crate::{DataCategory, DataQuery, DataRecord, DataRegistry, SensitivityLevel};
#[test]
fn query_min_level_filters_correctly() {
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
        SensitivityLevel::Restricted,
        DataCategory::Generic,
        0,
    ))
    .unwrap();
    let q = DataQuery::new().min_level(SensitivityLevel::Confidential);
    let results = q.run(reg.all());
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "r2");
}
