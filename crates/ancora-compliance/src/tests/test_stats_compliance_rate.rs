use crate::{ComplianceControl, ComplianceStats, ControlRegistry, ControlStatus, Framework};
#[test]
fn compliance_rate_zero_when_empty() {
    let reg = ControlRegistry::new();
    let stats = ComplianceStats::from_registry(&reg, &Framework::Fedramp);
    assert!((stats.compliance_rate()).abs() < 1e-10);
}
#[test]
fn compliance_rate_half_when_one_of_two() {
    let mut reg = ControlRegistry::new();
    let mut c1 = ComplianceControl::new("AC-1", Framework::Fedramp, "A", "B");
    c1.set_status(ControlStatus::Compliant, 1);
    reg.register(c1);
    let mut c2 = ComplianceControl::new("AU-2", Framework::Fedramp, "C", "D");
    c2.set_status(ControlStatus::NonCompliant, 2);
    reg.register(c2);
    let stats = ComplianceStats::from_registry(&reg, &Framework::Fedramp);
    assert!((stats.compliance_rate() - 0.5).abs() < 1e-10);
}
