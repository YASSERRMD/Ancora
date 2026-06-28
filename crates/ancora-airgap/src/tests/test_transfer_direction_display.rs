use crate::transfer::TransferDirection;

#[test]
fn display_inbound() {
    assert_eq!(format!("{}", TransferDirection::Inbound), "INBOUND");
}

#[test]
fn display_outbound() {
    assert_eq!(format!("{}", TransferDirection::Outbound), "OUTBOUND");
}
