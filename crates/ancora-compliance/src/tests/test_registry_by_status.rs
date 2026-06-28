use crate::{ComplianceControl, ControlRegistry, ControlStatus, Framework};
#[test]
fn by_status_filters_correctly() {
    let mut reg = ControlRegistry::new();
    let mut c1 = ComplianceControl::new("CC6.1", Framework::Soc2, "A", "B");
    c1.set_status(ControlStatus::Compliant, 1);
    reg.register(c1);
    reg.register(ComplianceControl::new("CC7.1", Framework::Soc2, "C", "D"));
    assert_eq!(reg.by_status(&ControlStatus::Compliant).len(), 1);
    assert_eq!(reg.by_status(&ControlStatus::NotAssessed).len(), 1);
}
