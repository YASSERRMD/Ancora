use crate::{ComplianceControl, ComplianceReport, ControlRegistry, ControlStatus, Framework};
#[test]
fn report_generate_counts_correctly() {
    let mut reg = ControlRegistry::new();
    let mut c1 = ComplianceControl::new("CC6.1", Framework::Soc2, "A", "B");
    c1.set_status(ControlStatus::Compliant, 1);
    reg.register(c1);
    reg.register(ComplianceControl::new("CC7.1", Framework::Soc2, "C", "D"));
    let report = ComplianceReport::generate(&reg, &Framework::Soc2, "t1", 100);
    assert_eq!(report.total_controls, 2);
    assert_eq!(report.compliant, 1);
    assert_eq!(report.not_assessed, 1);
}
