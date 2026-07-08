use crate::{DataCategory, DataRecord, SensitivityLevel};
#[test]
fn record_stores_all_fields() {
    let r = DataRecord::new(
        "id1",
        "t1",
        "Employee SSN",
        SensitivityLevel::Restricted,
        DataCategory::Pii,
        42,
    );
    assert_eq!(r.id, "id1");
    assert_eq!(r.tenant_id, "t1");
    assert_eq!(r.level, SensitivityLevel::Restricted);
    assert_eq!(r.category, DataCategory::Pii);
    assert_eq!(r.created_tick, 42);
}
