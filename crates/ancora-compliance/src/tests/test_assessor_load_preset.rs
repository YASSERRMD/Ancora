use crate::{AutoAssessor, ControlRegistry, Framework, presets};
#[test]
fn load_preset_populates_registry() {
    let mut reg = ControlRegistry::new();
    AutoAssessor::load_preset(&mut reg, presets::fedramp_controls());
    assert_eq!(reg.for_framework(&Framework::Fedramp).len(), 5);
}
