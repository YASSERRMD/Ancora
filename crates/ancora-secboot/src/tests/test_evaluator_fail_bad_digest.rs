use crate::{BootChain, BootPolicy, IntegrityDecision, IntegrityEvaluator, Measurement, MeasurementKind};
#[test]
fn evaluator_fails_when_digest_not_allowed() {
    let policy = BootPolicy::new("t1").allow_digest("vmlinuz", "good-digest");
    let mut chain = BootChain::new("t1", "n1");
    chain.add_step(Measurement::new("m1", MeasurementKind::Kernel, "vmlinuz", "bad-digest", 0));
    let decision = IntegrityEvaluator::evaluate(&policy, &chain);
    assert!(matches!(decision, IntegrityDecision::Fail(_)));
}
