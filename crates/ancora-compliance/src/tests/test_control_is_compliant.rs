use crate::{ComplianceControl, ControlStatus, Framework};
#[test]
fn is_compliant_only_for_compliant_status() {
    let mut c = ComplianceControl::new("CC6.1", Framework::Soc2, "A", "B");
    assert!(!c.is_compliant());
    c.set_status(ControlStatus::Compliant, 1);
    assert!(c.is_compliant());
    c.set_status(ControlStatus::PartiallyCompliant, 2);
    assert!(!c.is_compliant());
}
