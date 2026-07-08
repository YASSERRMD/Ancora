use crate::{DataCategory, DataRecord, DataRegistry, SensitivityLevel};
#[test]
fn remove_existing_record() {
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
    assert!(reg.remove("r1").is_ok());
    assert_eq!(reg.count(), 0);
    assert!(reg.remove("r1").is_err());
}
