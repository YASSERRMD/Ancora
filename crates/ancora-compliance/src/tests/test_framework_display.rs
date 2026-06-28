use crate::Framework;
#[test]
fn framework_display() {
    assert_eq!(format!("{}", Framework::Soc2), "SOC 2");
    assert_eq!(format!("{}", Framework::Fedramp), "FedRAMP");
    assert_eq!(format!("{}", Framework::Iso27001), "ISO 27001");
}
