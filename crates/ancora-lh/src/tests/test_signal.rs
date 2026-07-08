use crate::signal::{ExternalSignal, SignalQueue};

fn make_signal(run_id: &str, kind: &str) -> ExternalSignal {
    ExternalSignal {
        run_id: run_id.to_string(),
        kind: kind.to_string(),
        payload: "{}".to_string(),
        tick: 1,
    }
}

#[test]
fn external_signal_handled() {
    let mut q = SignalQueue::default();
    q.inject(make_signal("r1", "pause"));
    let s = q.pop().unwrap();
    assert_eq!(s.kind, "pause");
}

#[test]
fn signal_queue_pending_count() {
    let mut q = SignalQueue::default();
    q.inject(make_signal("r1", "a"));
    q.inject(make_signal("r1", "b"));
    assert_eq!(q.pending(), 2);
}

#[test]
fn empty_signal_queue_pops_none() {
    let mut q = SignalQueue::default();
    assert!(q.pop().is_none());
}
