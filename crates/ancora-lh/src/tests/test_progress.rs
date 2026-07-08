use crate::progress::ProgressStore;

#[test]
fn progress_persisted_across_advances() {
    let mut store = ProgressStore::default();
    store.init("r1", 4);
    store.advance("r1", 10);
    store.advance("r1", 20);
    let p = store.get("r1").unwrap();
    assert_eq!(p.steps_completed, 2);
    assert_eq!(p.last_tick, 20);
}

#[test]
fn progress_pct_complete_correct() {
    let mut store = ProgressStore::default();
    store.init("r1", 10);
    for i in 0..5 {
        store.advance("r1", i);
    }
    let p = store.get("r1").unwrap();
    assert_eq!(p.pct_complete(), 50.0);
}

#[test]
fn progress_does_not_exceed_total() {
    let mut store = ProgressStore::default();
    store.init("r1", 2);
    for i in 0..10 {
        store.advance("r1", i);
    }
    let p = store.get("r1").unwrap();
    assert_eq!(p.steps_completed, 2);
}
