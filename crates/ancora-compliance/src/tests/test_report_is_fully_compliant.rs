use crate::{ComplianceControl, ComplianceReport, ControlRegistry, ControlStatus, Framework};
#[test]
fn fully_compliant_only_when_no_gaps() {
    let mut reg = ControlRegistry::new();
    let mut c = ComplianceControl::new("CC6.1", Framework::Soc2, "A", "B");
    c.set_status(ControlStatus::Compliant, 1);
    reg.register(c);
    let report = ComplianceReport::generate(&reg, &Framework::Soc2, "t1", 2);
    assert!(report.is_fully_compliant());
}
#[test]
fn not_fully_compliant_when_non_compliant_exists() {
    let mut reg = ControlRegistry::new();
    let mut c = ComplianceControl::new("CC6.1", Framework::Soc2, "A", "B");
    c.set_status(ControlStatus::NonCompliant, 1);
    reg.register(c);
    let report = ComplianceReport::generate(&reg, &Framework::Soc2, "t1", 2);
    assert!(!report.is_fully_compliant());
}
