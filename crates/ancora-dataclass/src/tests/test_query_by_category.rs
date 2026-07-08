use crate::{DataCategory, DataQuery, DataRecord, DataRegistry, SensitivityLevel};
#[test]
fn query_by_category() {
    let mut reg = DataRegistry::new();
    reg.insert(DataRecord::new(
        "r1",
        "t1",
        "x",
        SensitivityLevel::Internal,
        DataCategory::Pii,
        0,
    ))
    .unwrap();
    reg.insert(DataRecord::new(
        "r2",
        "t1",
        "y",
        SensitivityLevel::Internal,
        DataCategory::Financial,
        0,
    ))
    .unwrap();
    let q = DataQuery::new().category(DataCategory::Pii);
    let results = q.run(reg.all());
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].category, DataCategory::Pii);
}
