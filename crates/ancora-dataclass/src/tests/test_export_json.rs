use crate::{DataCategory, DataRecord, SensitivityLevel, to_json};
#[test]
fn json_output_is_array() {
    let r = DataRecord::new("r1", "t1", "x", SensitivityLevel::Public, DataCategory::Generic, 5);
    let json = to_json(&[&r]);
    assert!(json.starts_with('['));
    assert!(json.ends_with(']'));
    assert!(json.contains("\"id\":\"r1\""));
    assert!(json.contains("\"level\":\"PUBLIC\""));
}
