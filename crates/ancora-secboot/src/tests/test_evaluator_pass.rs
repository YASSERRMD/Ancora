use crate::{BootChain, BootPolicy, IntegrityDecision, IntegrityEvaluator, Measurement, MeasurementKind};
#[test]
fn evaluator_passes_with_allowed_digest_and_required_kinds() {
    let policy = BootPolicy::new("t1")
        .require_kind(MeasurementKind::Kernel)
        .allow_digest("vmlinuz", "abc123");
    let mut chain = BootChain::new("t1", "n1");
    chain.add_step(Measurement::new("m1", MeasurementKind::Kernel, "vmlinuz", "abc123", 0));
    let decision = IntegrityEvaluator::evaluate(&policy, &chain);
    assert_eq!(decision, IntegrityDecision::Pass);
}
