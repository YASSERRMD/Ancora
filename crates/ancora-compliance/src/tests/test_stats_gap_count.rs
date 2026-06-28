use crate::{ComplianceControl, ComplianceStats, ControlRegistry, ControlStatus, Framework};
#[test]
fn gap_count_includes_non_compliant_and_not_assessed() {
    let mut reg = ControlRegistry::new();
    let mut c1 = ComplianceControl::new("AC-1", Framework::Fedramp, "A", "B");
    c1.set_status(ControlStatus::NonCompliant, 1);
    reg.register(c1);
    reg.register(ComplianceControl::new("AU-2", Framework::Fedramp, "C", "D"));
    let stats = ComplianceStats::from_registry(&reg, &Framework::Fedramp);
    assert_eq!(stats.gap_count(), 2);
}
