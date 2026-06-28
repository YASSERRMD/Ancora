use crate::{DataCategory, DataRecord, SensitivityLevel};
#[test]
fn with_tag_and_has_tag() {
    let r = DataRecord::new("id1", "t1", "x", SensitivityLevel::Internal, DataCategory::Generic, 0)
        .with_tag("gdpr");
    assert!(r.has_tag("gdpr"));
    assert!(!r.has_tag("hipaa"));
}
