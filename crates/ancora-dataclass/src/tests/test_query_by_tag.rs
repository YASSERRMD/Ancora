use crate::{DataCategory, DataQuery, DataRecord, DataRegistry, SensitivityLevel};
#[test]
fn query_by_tag() {
    let mut reg = DataRegistry::new();
    let r1 = DataRecord::new(
        "r1",
        "t1",
        "x",
        SensitivityLevel::Internal,
        DataCategory::Generic,
        0,
    )
    .with_tag("gdpr");
    let r2 = DataRecord::new(
        "r2",
        "t1",
        "y",
        SensitivityLevel::Internal,
        DataCategory::Generic,
        0,
    );
    reg.insert(r1).unwrap();
    reg.insert(r2).unwrap();
    let q = DataQuery::new().tag("gdpr");
    let results = q.run(reg.all());
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "r1");
}
