/// Tests verifying rollback restores the prior model.
use crate::runtime::{make_model, SwapRuntime};

#[test]
fn test_rollback_restores_correct_version() {
    let m1 = make_model("r-v1");
    let m2 = make_model("r-v2");
    let v1 = m1.version();

    let rt = SwapRuntime::new(m1);
    rt.swap(m2);

    let rb = rt.rollback().expect("rollback should succeed");
    assert_eq!(rb.restored_version, v1);
    assert_eq!(rt.active_model().version(), v1);
}

#[test]
fn test_rollback_journal_has_two_entries() {
    let m1 = make_model("rb-j1");
    let m2 = make_model("rb-j2");
    let rt = SwapRuntime::new(m1);
    rt.swap(m2);
    rt.rollback().unwrap();

    let j = rt.journal();
    assert_eq!(j.len(), 2);
}

#[test]
fn test_no_rollback_without_prior_swap() {
    let m = make_model("solo");
    let rt = SwapRuntime::new(m);
    assert!(rt.rollback().is_err());
}
