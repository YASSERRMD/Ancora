use crate::{ComplianceControl, ComplianceStats, ControlRegistry, ControlStatus, Framework};
#[test]
fn stats_from_registry_counts_correctly() {
    let mut reg = ControlRegistry::new();
    let mut c1 = ComplianceControl::new("CC6.1", Framework::Soc2, "A", "B");
    c1.set_status(ControlStatus::Compliant, 1);
    reg.register(c1);
    reg.register(ComplianceControl::new("CC7.1", Framework::Soc2, "C", "D"));
    let stats = ComplianceStats::from_registry(&reg, &Framework::Soc2);
    assert_eq!(stats.total, 2);
    assert_eq!(stats.compliant, 1);
    assert_eq!(stats.not_assessed, 1);
}
