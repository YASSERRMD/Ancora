use crate::{BootChain, Measurement, MeasurementKind};
#[test]
fn chain_adds_steps_in_order() {
    let mut chain = BootChain::new("t1", "node1");
    chain.add_step(Measurement::new("m1", MeasurementKind::Firmware, "fw", "d1", 0));
    chain.add_step(Measurement::new("m2", MeasurementKind::Kernel, "k", "d2", 1));
    assert_eq!(chain.len(), 2);
    assert!(!chain.is_empty());
}
