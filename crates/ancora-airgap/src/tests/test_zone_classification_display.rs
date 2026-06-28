use crate::boundary::ZoneClassification;

#[test]
fn display_public() {
    assert_eq!(format!("{}", ZoneClassification::Public), "PUBLIC");
}

#[test]
fn display_internal() {
    assert_eq!(format!("{}", ZoneClassification::Internal), "INTERNAL");
}

#[test]
fn display_restricted() {
    assert_eq!(format!("{}", ZoneClassification::Restricted), "RESTRICTED");
}

#[test]
fn display_top_secret() {
    assert_eq!(format!("{}", ZoneClassification::TopSecret), "TOP_SECRET");
}
