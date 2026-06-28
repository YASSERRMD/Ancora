use crate::{BootChain, Measurement, MeasurementKind};
#[test]
fn present_kinds_aggregates_step_kinds() {
    let mut chain = BootChain::new("t1", "n1");
    chain.add_step(Measurement::new("m1", MeasurementKind::Firmware, "fw", "d1", 0));
    chain.add_step(Measurement::new("m2", MeasurementKind::Kernel, "k", "d2", 1));
    let kinds = chain.present_kinds();
    assert!(kinds.contains("FIRMWARE"));
    assert!(kinds.contains("KERNEL"));
}
