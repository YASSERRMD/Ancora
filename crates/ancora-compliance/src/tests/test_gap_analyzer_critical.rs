use crate::{ComplianceControl, ControlRegistry, ControlStatus, Framework, GapAnalyzer};
#[test]
fn critical_gaps_requires_non_compliant_and_no_evidence() {
    let mut reg = ControlRegistry::new();
    let mut c = ComplianceControl::new("CC6.1", Framework::Soc2, "A", "B");
    c.set_status(ControlStatus::NonCompliant, 1);
    reg.register(c);
    let critical = GapAnalyzer::critical_gaps(&reg, &Framework::Soc2);
    assert_eq!(critical.len(), 1);
}
#[test]
fn critical_gaps_empty_when_evidence_attached() {
    let mut reg = ControlRegistry::new();
    let mut c = ComplianceControl::new("CC6.1", Framework::Soc2, "A", "B");
    c.set_status(ControlStatus::NonCompliant, 1);
    c.attach_evidence("ev-001");
    reg.register(c);
    let critical = GapAnalyzer::critical_gaps(&reg, &Framework::Soc2);
    assert!(critical.is_empty());
}
