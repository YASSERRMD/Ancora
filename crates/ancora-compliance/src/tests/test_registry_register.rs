use crate::{ComplianceControl, ControlRegistry, Framework};
#[test]
fn registry_register_and_count() {
    let mut reg = ControlRegistry::new();
    reg.register(ComplianceControl::new("CC6.1", Framework::Soc2, "A", "B"));
    assert_eq!(reg.count(), 1);
}
