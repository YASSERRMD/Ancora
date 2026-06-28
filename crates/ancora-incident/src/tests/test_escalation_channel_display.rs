use crate::escalation::EscalationChannel;

#[test]
fn channel_display() {
    assert_eq!(format!("{}", EscalationChannel::Pager), "PAGER");
    assert_eq!(format!("{}", EscalationChannel::Email), "EMAIL");
    assert_eq!(format!("{}", EscalationChannel::Chat), "CHAT");
    assert_eq!(format!("{}", EscalationChannel::Phone), "PHONE");
}
