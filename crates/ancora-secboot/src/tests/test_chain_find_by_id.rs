use crate::{BootChain, Measurement, MeasurementKind};
#[test]
fn find_by_id_returns_step() {
    let mut chain = BootChain::new("t1", "n1");
    chain.add_step(Measurement::new("m1", MeasurementKind::Kernel, "k", "d1", 0));
    assert!(chain.find_by_id("m1").is_some());
    assert!(chain.find_by_id("missing").is_none());
}
