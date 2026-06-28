use crate::DataCategory;
#[test]
fn category_display_values() {
    assert_eq!(format!("{}", DataCategory::Pii), "PII");
    assert_eq!(format!("{}", DataCategory::Financial), "FINANCIAL");
    assert_eq!(format!("{}", DataCategory::Credentials), "CREDENTIALS");
}
