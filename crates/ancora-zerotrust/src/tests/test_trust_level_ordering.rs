use crate::device::TrustLevel;

#[test]
fn trust_level_order() {
    assert!(TrustLevel::Untrusted < TrustLevel::Partial);
    assert!(TrustLevel::Partial < TrustLevel::Trusted);
    assert!(TrustLevel::Trusted < TrustLevel::FullyTrusted);
}
