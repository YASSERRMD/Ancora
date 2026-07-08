use crate::{DataCategory, DataRecord, SensitivityLevel};
#[test]
fn with_metadata_stores_key_value() {
    let r = DataRecord::new(
        "id1",
        "t1",
        "x",
        SensitivityLevel::Internal,
        DataCategory::Generic,
        0,
    )
    .with_metadata("owner", "alice");
    assert_eq!(r.metadata.get("owner").map(String::as_str), Some("alice"));
}
