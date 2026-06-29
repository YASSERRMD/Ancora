/// Tests verifying that in-flight runs keep their pinned model after a swap.

use crate::runtime::{make_model, RunId, SwapRuntime};

#[test]
fn test_in_flight_run_version_unchanged_after_swap() {
    let m1 = make_model("pinned-v1");
    let rt = SwapRuntime::new(m1.clone());

    let run = RunId(100);
    rt.start_run(run).unwrap();
    let original_ver = rt.run_model_version(run).unwrap();

    let m2 = make_model("pinned-v2");
    rt.swap(m2.clone());

    // Version pinned to run must not change.
    assert_eq!(rt.run_model_version(run).unwrap(), original_ver);
    assert_eq!(original_ver, m1.version());

    rt.finish_run(run);
}

#[test]
fn test_multiple_runs_different_pins() {
    let m1 = make_model("base");
    let rt = SwapRuntime::new(m1.clone());

    let run1 = RunId(201);
    let run2 = RunId(202);
    rt.start_run(run1).unwrap();

    // Swap before run2 starts.
    let m2 = make_model("next");
    rt.swap(m2.clone());

    rt.start_run(run2).unwrap();

    // run1 pinned to m1, run2 pinned to m2.
    assert_eq!(rt.run_model_version(run1).unwrap(), m1.version());
    assert_eq!(rt.run_model_version(run2).unwrap(), m2.version());

    rt.finish_run(run1);
    rt.finish_run(run2);
}
