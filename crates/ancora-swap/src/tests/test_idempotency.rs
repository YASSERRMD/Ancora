/// Tests verifying zero duplicate effects across a swap sequence.

use crate::runtime::{make_model, RunId, SwapRuntime};

#[test]
fn test_no_double_unload() {
    let m1 = make_model("dup-base");
    let rt = SwapRuntime::new(m1.clone());

    let m2 = make_model("dup-new");
    rt.swap(m2.clone());

    // Calling unload a second time must be idempotent.
    m1.unload();
    assert!(m1.is_unloaded());
    // Pin count not affected by double-unload.
    assert_eq!(m1.pin_count(), 0);
    assert!(m1.can_reclaim());
}

#[test]
fn test_no_double_journal_entry_on_same_swap() {
    let m1 = make_model("j-dup1");
    let rt = SwapRuntime::new(m1);

    let m2 = make_model("j-dup2");
    rt.swap(m2);

    // Exactly one journal entry.
    assert_eq!(rt.journal().len(), 1);
}

#[test]
fn test_finish_run_idempotent() {
    let m = make_model("fin-idem");
    let rt = SwapRuntime::new(m);

    let run = RunId(500);
    rt.start_run(run).unwrap();
    rt.finish_run(run);
    // Second finish_run must not panic.
    rt.finish_run(run);
    assert_eq!(rt.active_run_count(), 0);
}

#[test]
fn test_reclaim_idempotent_when_empty() {
    let m = make_model("rec-idem");
    let rt = SwapRuntime::new(m);
    // Nothing to reclaim.
    let freed = rt.reclaim_unloaded();
    assert_eq!(freed, 0);
    let freed2 = rt.reclaim_unloaded();
    assert_eq!(freed2, 0);
}
