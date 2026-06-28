use crate::{DataCategory, DataRecordBuilder, SensitivityLevel};
#[test]
fn builder_sets_all_fields() {
    let r = DataRecordBuilder::new("r1", "t1", "SSN")
        .level(SensitivityLevel::Restricted)
        .category(DataCategory::Pii)
        .tick(100)
        .tag("gdpr")
        .tag("hipaa")
        .build();
    assert_eq!(r.level, SensitivityLevel::Restricted);
    assert_eq!(r.category, DataCategory::Pii);
    assert_eq!(r.created_tick, 100);
    assert!(r.has_tag("gdpr"));
    assert!(r.has_tag("hipaa"));
}
