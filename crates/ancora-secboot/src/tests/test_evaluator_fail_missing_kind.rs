use crate::{
    BootChain, BootPolicy, IntegrityDecision, IntegrityEvaluator, Measurement, MeasurementKind,
};
#[test]
fn evaluator_fails_when_required_kind_absent() {
    let policy = BootPolicy::new("t1")
        .require_kind(MeasurementKind::Firmware)
        .allow_unknown();
    let mut chain = BootChain::new("t1", "n1");
    chain.add_step(Measurement::new(
        "m1",
        MeasurementKind::Kernel,
        "k",
        "d1",
        0,
    ));
    let decision = IntegrityEvaluator::evaluate(&policy, &chain);
    assert!(matches!(decision, IntegrityDecision::Fail(_)));
}
