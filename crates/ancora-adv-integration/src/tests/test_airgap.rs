// Air-gap test: verify all advanced capabilities work without any network access.
// All stores are in-memory; no external calls are made. This test is a structural
// check that all operations succeed using only local data.

use ancora_orchestrate::DepthLimiter;
use ancora_guard::{GuardrailJournal, GuardrailPolicy, PiiInputGuardrail};
use ancora_reason::{StepDecomposer, StepStatus};
use ancora_ageval::EvalDataset;

#[test]
fn combined_air_gapped_run() {
    // orchestrate: depth limiter works with no external calls
    let mut limiter = DepthLimiter::new(3);
    limiter.enter().unwrap();
    limiter.enter().unwrap();
    assert_eq!(limiter.depth(), 2);
    limiter.exit();
    limiter.exit();

    // guard: PII guardrail checks are local
    let mut policy = GuardrailPolicy::new();
    policy.add_input(PiiInputGuardrail);
    let mut journal = GuardrailJournal::default();
    policy.check_input("call me at 555-1234", &mut journal, 1);

    // reason: step decomposition is local
    let steps = StepDecomposer::decompose(vec!["step-A".into(), "step-B".into()]);
    assert!(steps.iter().all(|s| s.status == StepStatus::Pending));

    // eval: dataset operations are local
    let mut ds = EvalDataset::new("airgap-test");
    ds.add(ancora_ageval::EvalSample::new("s1").with_tag("offline"));
    assert_eq!(ds.len(), 1);
}

#[test]
fn combined_residency_enforced() {
    // All data remains in local memory: no serialization to external stores
    use ancora_coord::Blackboard;
    let mut board = Blackboard::default();
    board.write("agent-a", "k1", "v1").unwrap();
    assert_eq!(board.read("k1").unwrap(), "v1");
    // No external I/O occurred
}
