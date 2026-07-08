use crate::{DataCategory, DataRecord, DataRegistry, SensitivityLevel};
#[test]
fn by_tenant_filters_correctly() {
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
        "t2",
        "y",
        SensitivityLevel::Internal,
        DataCategory::Generic,
        0,
    ))
    .unwrap();
    assert_eq!(reg.by_tenant("t1").len(), 1);
    assert_eq!(reg.by_tenant("t2").len(), 1);
    assert_eq!(reg.by_tenant("t3").len(), 0);
}
