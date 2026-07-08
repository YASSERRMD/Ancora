use crate::{DataCategory, DataRecord, DataRegistry, SensitivityLevel};
#[test]
fn insert_succeeds_for_new_record() {
    let mut reg = DataRegistry::new();
    let r = DataRecord::new(
        "r1",
        "t1",
        "x",
        SensitivityLevel::Internal,
        DataCategory::Generic,
        0,
    );
    assert!(reg.insert(r).is_ok());
    assert_eq!(reg.count(), 1);
}
#[test]
fn insert_fails_for_duplicate_id() {
    let mut reg = DataRegistry::new();
    let r1 = DataRecord::new(
        "dup",
        "t1",
        "x",
        SensitivityLevel::Internal,
        DataCategory::Generic,
        0,
    );
    let r2 = DataRecord::new(
        "dup",
        "t1",
        "y",
        SensitivityLevel::Internal,
        DataCategory::Generic,
        0,
    );
    reg.insert(r1).unwrap();
    assert!(reg.insert(r2).is_err());
}
