use crate::{Framework, presets};
#[test]
fn soc2_preset_returns_five_controls() {
    let controls = presets::soc2_controls();
    assert_eq!(controls.len(), 5);
    assert!(controls.iter().all(|c| c.framework == Framework::Soc2));
}
