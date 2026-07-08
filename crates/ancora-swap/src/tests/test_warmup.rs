/// Tests verifying warmup completes before the model serves traffic.
use crate::runtime::{make_model, WarmupStatus};
use crate::warmup::WarmupGate;

#[test]
fn test_warmup_gate_not_ready_before_run() {
    let m = make_model("wg");
    let gate = WarmupGate::new(m);
    // Before run(), the gate is in InProgress state.
    assert!(!gate.is_ready());
    assert!(matches!(gate.status(), WarmupStatus::InProgress));
}

#[test]
fn test_warmup_gate_ready_after_run() {
    let m = make_model("wg-done");
    let mut gate = WarmupGate::new(m);
    gate.run(1, 0); // 1 synthetic prompt, 0 ms latency
    assert!(gate.is_ready());
}

#[test]
fn test_warmup_gate_into_handle_fails_if_not_ready() {
    let m = make_model("wg-fail");
    let gate = WarmupGate::new(m);
    assert!(gate.into_handle().is_err());
}

#[test]
fn test_warmup_gate_into_handle_succeeds_after_warmup() {
    let m = make_model("wg-ok");
    let v = m.version();
    let mut gate = WarmupGate::new(m);
    gate.run(0, 0);
    let handle = gate.into_handle().expect("should succeed after warmup");
    assert_eq!(handle.version(), v);
}

#[test]
fn test_runtime_warmup_returns_complete() {
    let base = make_model("rt-warmup-base");
    let rt = crate::runtime::SwapRuntime::new(base);
    let candidate = make_model("rt-warmup-cand");
    let status = rt.warmup(&candidate, 0);
    assert!(matches!(status, WarmupStatus::Complete(_)));
}
