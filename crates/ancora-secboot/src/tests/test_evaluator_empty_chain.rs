use crate::{BootChain, BootPolicy, IntegrityDecision, IntegrityEvaluator};
#[test]
fn evaluator_fails_on_empty_chain() {
    let policy = BootPolicy::new("t1").allow_unknown();
    let chain = BootChain::new("t1", "n1");
    let decision = IntegrityEvaluator::evaluate(&policy, &chain);
    assert!(matches!(decision, IntegrityDecision::Fail(_)));
}
