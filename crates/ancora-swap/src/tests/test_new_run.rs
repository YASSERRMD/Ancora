/// Tests that new runs started after a swap use the new model.
use crate::runtime::{make_model, RunId, SwapRuntime};

#[test]
fn test_new_run_after_swap_uses_new_model() {
    let m1 = make_model("old");
    let rt = SwapRuntime::new(m1);

    let m2 = make_model("new");
    let v2 = m2.version();
    rt.swap(m2);

    let run = RunId(300);
    rt.start_run(run).unwrap();
    assert_eq!(rt.run_model_version(run).unwrap(), v2);
    rt.finish_run(run);
}

#[test]
fn test_sequential_swaps_new_run_gets_latest() {
    let m1 = make_model("gen1");
    let rt = SwapRuntime::new(m1);

    let m2 = make_model("gen2");
    rt.swap(m2);
    rt.reclaim_unloaded();

    let m3 = make_model("gen3");
    let v3 = m3.version();
    rt.swap(m3);

    let run = RunId(301);
    rt.start_run(run).unwrap();
    assert_eq!(rt.run_model_version(run).unwrap(), v3);
    rt.finish_run(run);
}
