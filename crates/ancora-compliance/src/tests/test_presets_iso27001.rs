use crate::{presets, Framework};
#[test]
fn iso27001_preset_returns_five_controls() {
    let controls = presets::iso27001_controls();
    assert_eq!(controls.len(), 5);
    assert!(controls.iter().all(|c| c.framework == Framework::Iso27001));
}
