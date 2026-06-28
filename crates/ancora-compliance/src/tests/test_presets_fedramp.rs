use crate::{Framework, presets};
#[test]
fn fedramp_preset_returns_five_controls() {
    let controls = presets::fedramp_controls();
    assert_eq!(controls.len(), 5);
    assert!(controls.iter().all(|c| c.framework == Framework::Fedramp));
}
