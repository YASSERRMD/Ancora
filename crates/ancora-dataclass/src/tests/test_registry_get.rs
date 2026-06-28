use crate::{DataCategory, DataRecord, DataRegistry, SensitivityLevel};
#[test]
fn get_existing_record() {
    let mut reg = DataRegistry::new();
    reg.insert(DataRecord::new("r1", "t1", "x", SensitivityLevel::Internal, DataCategory::Generic, 0)).unwrap();
    assert!(reg.get("r1").is_ok());
    assert!(reg.get("missing").is_err());
}
