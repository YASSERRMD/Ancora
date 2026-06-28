use crate::{ComplianceControl, ControlStatus, Framework};
#[test]
fn set_status_updates_status_and_tick() {
    let mut c = ComplianceControl::new("CC6.1", Framework::Soc2, "Access", "desc");
    c.set_status(ControlStatus::Compliant, 42);
    assert_eq!(c.status, ControlStatus::Compliant);
    assert_eq!(c.assessed_tick, Some(42));
}
