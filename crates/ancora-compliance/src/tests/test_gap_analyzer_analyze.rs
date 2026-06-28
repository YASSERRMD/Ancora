use crate::{ComplianceControl, ControlRegistry, ControlStatus, Framework, GapAnalyzer};
#[test]
fn analyze_returns_non_compliant_and_not_assessed() {
    let mut reg = ControlRegistry::new();
    let mut c1 = ComplianceControl::new("CC6.1", Framework::Soc2, "A", "B");
    c1.set_status(ControlStatus::NonCompliant, 1);
    let mut c2 = ComplianceControl::new("CC7.1", Framework::Soc2, "C", "D");
    c2.set_status(ControlStatus::Compliant, 2);
    reg.register(c1);
    reg.register(c2);
    let gaps = GapAnalyzer::analyze(&reg, &Framework::Soc2);
    assert_eq!(gaps.len(), 1);
    assert_eq!(gaps[0].control_id, "CC6.1");
}
