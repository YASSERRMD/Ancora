use crate::runtime::{make_model, RunId, SwapRuntime, WarmupStatus};

#[test]
fn test_hot_swap_keeps_runs_alive() {
    let m1 = make_model("m1");
    let rt = SwapRuntime::new(m1.clone());

    let run = RunId(1);
    rt.start_run(run).expect("run should start");

    let m2 = make_model("m2");
    let result = rt.swap(m2.clone());

    // Run must still hold a pin on m1 (old model).
    assert!(m1.pin_count() >= 1, "in-flight run still pins old model");
    // Active model is now m2.
    assert_eq!(rt.active_model().version(), m2.version());
    assert!(result.elapsed_ns < u64::MAX);

    rt.finish_run(run);
    // After run finishes and old model was unloaded during swap, it can be reclaimed.
    assert!(m1.can_reclaim());
}

#[test]
fn test_in_flight_run_keeps_pinned_model() {
    let m1 = make_model("alpha");
    let rt = SwapRuntime::new(m1.clone());

    let run = RunId(42);
    rt.start_run(run).unwrap();

    // Swap to m2.
    let m2 = make_model("beta");
    rt.swap(m2.clone());

    // Run still sees its original model version.
    let run_ver = rt
        .run_model_version(run)
        .expect("run should have a version");
    assert_eq!(
        run_ver,
        m1.version(),
        "run must be pinned to the original model"
    );

    rt.finish_run(run);
}

#[test]
fn test_new_run_uses_swapped_model() {
    let m1 = make_model("v1");
    let rt = SwapRuntime::new(m1);

    let m2 = make_model("v2");
    rt.swap(m2.clone());

    let run = RunId(99);
    rt.start_run(run).expect("new run on swapped model");

    let run_ver = rt.run_model_version(run).unwrap();
    assert_eq!(run_ver, m2.version(), "new run must use swapped model");

    rt.finish_run(run);
}

#[test]
fn test_rollback_restores_prior_model() {
    let m1 = make_model("original");
    let rt = SwapRuntime::new(m1.clone());

    let m2 = make_model("replacement");
    rt.swap(m2);

    let rb = rt.rollback().expect("rollback must succeed");
    assert_eq!(rb.restored_version, m1.version());
    assert_eq!(rt.active_model().version(), m1.version());
}

#[test]
fn test_rollback_fails_without_drain() {
    let m1 = make_model("solo");
    let rt = SwapRuntime::new(m1);
    // No swap was performed, so there is nothing to roll back.
    assert!(rt.rollback().is_err());
}

#[test]
fn test_warmup_completes_before_serving() {
    let m1 = make_model("base");
    let rt = SwapRuntime::new(m1);

    let candidate = make_model("candidate");
    // Use 0 ms so the test is fast.
    let status = rt.warmup(&candidate, 0);
    assert!(
        matches!(status, WarmupStatus::Complete(_)),
        "warmup must complete successfully"
    );
}

#[test]
fn test_memory_reclaimed_after_unload() {
    let m1 = make_model("to-unload");
    let rt = SwapRuntime::new(m1.clone());

    let m2 = make_model("new");
    rt.swap(m2);

    // No in-flight runs on m1, so draining model can be reclaimed immediately.
    let reclaimed = rt.reclaim_unloaded();
    assert_eq!(reclaimed, 1, "one model should be reclaimed");
    assert!(
        !rt.is_draining(),
        "drain slot should be empty after reclaim"
    );
}

#[test]
fn test_zero_duplicate_effects_across_swap() {
    // Verify that swapping twice does not double-journal the first swap.
    let m1 = make_model("a");
    let rt = SwapRuntime::new(m1);

    let m2 = make_model("b");
    rt.swap(m2);

    let journal_after_first = rt.journal();
    assert_eq!(journal_after_first.len(), 1);

    // Finish any draining so we can swap again.
    rt.reclaim_unloaded();

    let m3 = make_model("c");
    rt.swap(m3);

    let journal_after_second = rt.journal();
    assert_eq!(journal_after_second.len(), 2, "exactly two swap events");
}

#[test]
fn test_active_run_count() {
    let m = make_model("m");
    let rt = SwapRuntime::new(m);

    assert_eq!(rt.active_run_count(), 0);
    rt.start_run(RunId(1)).unwrap();
    rt.start_run(RunId(2)).unwrap();
    assert_eq!(rt.active_run_count(), 2);
    rt.finish_run(RunId(1));
    assert_eq!(rt.active_run_count(), 1);
    rt.finish_run(RunId(2));
    assert_eq!(rt.active_run_count(), 0);
}

#[test]
fn test_graceful_drain_old_runs_finish_after_swap() {
    let m1 = make_model("drain-old");
    let rt = SwapRuntime::new(m1.clone());

    // Start run before swap.
    let run = RunId(7);
    rt.start_run(run).unwrap();

    // Swap — m1 goes to draining.
    let m2 = make_model("drain-new");
    rt.swap(m2.clone());
    assert!(rt.is_draining(), "drain slot must be set after swap");

    // Old run still running — reclaim should not free m1 yet.
    let freed = rt.reclaim_unloaded();
    assert_eq!(freed, 0, "cannot reclaim while run holds a pin");

    // Finish old run — either finish_run auto-reclaims or reclaim_unloaded does.
    rt.finish_run(run);
    // After finishing, drain should be cleared (either eagerly or on next reclaim call).
    // Call reclaim to ensure any remaining drain is swept.
    let _ = rt.reclaim_unloaded();
    assert!(
        !rt.is_draining(),
        "drain slot should be empty after run finished"
    );
}
