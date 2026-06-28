use crate::{ComplianceControl, ComplianceReport, ControlRegistry, ControlStatus, Framework};
#[test]
fn compliance_rate_is_zero_when_all_not_assessed() {
    let mut reg = ControlRegistry::new();
    reg.register(ComplianceControl::new("CC6.1", Framework::Soc2, "A", "B"));
    let report = ComplianceReport::generate(&reg, &Framework::Soc2, "t1", 1);
    assert!((report.compliance_rate()).abs() < 1e-10);
}
#[test]
fn compliance_rate_is_one_when_all_compliant() {
    let mut reg = ControlRegistry::new();
    let mut c = ComplianceControl::new("CC6.1", Framework::Soc2, "A", "B");
    c.set_status(ControlStatus::Compliant, 1);
    reg.register(c);
    let report = ComplianceReport::generate(&reg, &Framework::Soc2, "t1", 2);
    assert!((report.compliance_rate() - 1.0).abs() < 1e-10);
}
