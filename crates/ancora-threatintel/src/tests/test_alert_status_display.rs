use crate::alert::AlertStatus;

#[test]
fn alert_status_display() {
    assert_eq!(format!("{}", AlertStatus::Open), "OPEN");
    assert_eq!(format!("{}", AlertStatus::Acknowledged), "ACKNOWLEDGED");
    assert_eq!(format!("{}", AlertStatus::Suppressed), "SUPPRESSED");
    assert_eq!(format!("{}", AlertStatus::Closed), "CLOSED");
}
