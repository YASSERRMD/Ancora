use crate::{DataCategory, DataQuery, DataRecord, DataRegistry, SensitivityLevel};
#[test]
fn query_by_exact_level() {
    let mut reg = DataRegistry::new();
    reg.insert(DataRecord::new(
        "r1",
        "t1",
        "x",
        SensitivityLevel::Internal,
        DataCategory::Generic,
        0,
    ))
    .unwrap();
    reg.insert(DataRecord::new(
        "r2",
        "t1",
        "y",
        SensitivityLevel::Confidential,
        DataCategory::Generic,
        0,
    ))
    .unwrap();
    let q = DataQuery::new().level(SensitivityLevel::Internal);
    let results = q.run(reg.all());
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "r1");
}
