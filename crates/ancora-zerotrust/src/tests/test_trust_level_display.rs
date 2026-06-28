use crate::device::TrustLevel;

#[test]
fn trust_level_display() {
    assert_eq!(format!("{}", TrustLevel::Untrusted), "UNTRUSTED");
    assert_eq!(format!("{}", TrustLevel::Partial), "PARTIAL");
    assert_eq!(format!("{}", TrustLevel::Trusted), "TRUSTED");
    assert_eq!(format!("{}", TrustLevel::FullyTrusted), "FULLY_TRUSTED");
}
