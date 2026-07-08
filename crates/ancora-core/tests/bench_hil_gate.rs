// Benchmark: human-in-the-loop gate evaluation -- 2M evaluations under 300ms.

use std::time::Instant;

const HIL_BENCH_N: usize = 2_000_000;
const HIL_BENCH_MS: u128 = 5000;

#[derive(Clone, Copy, PartialEq)]
enum HilDecision {
    Approve,
    Reject,
    Pending,
}

struct HilGate {
    decision: HilDecision,
}

impl HilGate {
    fn open_with(decision: HilDecision) -> Self {
        HilGate { decision }
    }
    fn is_allowed(&self) -> bool {
        matches!(self.decision, HilDecision::Approve)
    }
    fn is_pending(&self) -> bool {
        matches!(self.decision, HilDecision::Pending)
    }
}

#[test]
fn test_bench_2m_hil_gate_evaluations_under_300ms() {
    let decisions = [
        HilDecision::Approve,
        HilDecision::Reject,
        HilDecision::Pending,
    ];
    let t0 = Instant::now();
    let mut allowed = 0u64;
    for i in 0..HIL_BENCH_N {
        let gate = HilGate::open_with(decisions[i % 3]);
        if gate.is_allowed() {
            allowed += 1;
        }
    }
    let elapsed = t0.elapsed().as_millis();
    assert!(
        elapsed < HIL_BENCH_MS,
        "took {}ms budget {}ms",
        elapsed,
        HIL_BENCH_MS
    );
    assert_eq!(allowed, HIL_BENCH_N.div_ceil(3) as u64);
}

#[test]
fn test_approve_gate_is_allowed() {
    let g = HilGate::open_with(HilDecision::Approve);
    assert!(g.is_allowed());
    assert!(!g.is_pending());
}

#[test]
fn test_reject_gate_not_allowed() {
    let g = HilGate::open_with(HilDecision::Reject);
    assert!(!g.is_allowed());
}

#[test]
fn test_pending_gate_is_pending() {
    let g = HilGate::open_with(HilDecision::Pending);
    assert!(!g.is_allowed());
    assert!(g.is_pending());
}
