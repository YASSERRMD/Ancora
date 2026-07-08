use crate::{presets, AutoAssessor, ComplianceAuditLog, ControlRegistry, ControlStatus, Framework};
#[test]
fn bulk_mark_compliant_updates_registry_and_audit() {
    let mut reg = ControlRegistry::new();
    let mut audit = ComplianceAuditLog::new();
    AutoAssessor::load_preset(&mut reg, presets::soc2_controls());
    let ids = ["CC6.1", "CC6.2"];
    let results = AutoAssessor::bulk_mark_compliant(
        &mut reg,
        &mut audit,
        &ids,
        &Framework::Soc2,
        "t1",
        "alice",
        100,
    );
    assert_eq!(results.len(), 2);
    assert_eq!(audit.count(), 2);
    for id in &ids {
        assert!(reg
            .get(&crate::ControlId::new(*id))
            .is_some_and(|c| c.status == ControlStatus::Compliant));
    }
}
