use crate::indicator::IndicatorKind;

#[test]
fn kind_display() {
    assert_eq!(format!("{}", IndicatorKind::IpAddress), "IP_ADDRESS");
    assert_eq!(format!("{}", IndicatorKind::Domain), "DOMAIN");
    assert_eq!(format!("{}", IndicatorKind::Url), "URL");
    assert_eq!(format!("{}", IndicatorKind::FileHash), "FILE_HASH");
    assert_eq!(format!("{}", IndicatorKind::Email), "EMAIL");
    assert_eq!(format!("{}", IndicatorKind::CertificateHash), "CERT_HASH");
}
