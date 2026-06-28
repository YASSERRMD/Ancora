use crate::{BootChain, ChainStatus, Measurement, MeasurementKind};
#[test]
fn empty_chain_is_incomplete() {
    let chain = BootChain::new("t1", "n1");
    assert_eq!(chain.status(), ChainStatus::Incomplete);
}
#[test]
fn chain_with_steps_is_valid() {
    let mut chain = BootChain::new("t1", "n1");
    chain.add_step(Measurement::new("m1", MeasurementKind::Kernel, "k", "d", 0));
    assert_eq!(chain.status(), ChainStatus::Valid);
}
