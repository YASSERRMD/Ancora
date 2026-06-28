use crate::{ComplianceControl, ControlStatus, Framework};
#[test]
fn control_new_defaults_to_not_assessed() {
    let c = ComplianceControl::new("CC6.1", Framework::Soc2, "Access", "Access controls");
    assert_eq!(c.status, ControlStatus::NotAssessed);
    assert_eq!(c.evidence_ids.len(), 0);
    assert!(c.assessed_tick.is_none());
}
