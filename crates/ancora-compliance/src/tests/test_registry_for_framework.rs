use crate::{ComplianceControl, ControlRegistry, Framework};
#[test]
fn for_framework_filters_correctly() {
    let mut reg = ControlRegistry::new();
    reg.register(ComplianceControl::new("CC6.1", Framework::Soc2, "A", "B"));
    reg.register(ComplianceControl::new("A.5.1", Framework::Iso27001, "C", "D"));
    assert_eq!(reg.for_framework(&Framework::Soc2).len(), 1);
    assert_eq!(reg.for_framework(&Framework::Fedramp).len(), 0);
}
