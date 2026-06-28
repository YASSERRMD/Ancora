use crate::{ComplianceControl, ControlRegistry, ControlStatus, Framework};
#[test]
fn compliant_and_non_compliant_counts() {
    let mut reg = ControlRegistry::new();
    let mut c1 = ComplianceControl::new("CC6.1", Framework::Soc2, "A", "B");
    c1.set_status(ControlStatus::Compliant, 1);
    let mut c2 = ComplianceControl::new("CC7.1", Framework::Soc2, "C", "D");
    c2.set_status(ControlStatus::NonCompliant, 2);
    reg.register(c1);
    reg.register(c2);
    assert_eq!(reg.compliant_count(), 1);
    assert_eq!(reg.non_compliant_count(), 1);
}
