use crate::Protocol;
#[test]
fn protocol_variants_are_distinct() {
    assert_ne!(Protocol::Tcp, Protocol::Udp);
    assert_ne!(Protocol::Tcp, Protocol::Any);
}
