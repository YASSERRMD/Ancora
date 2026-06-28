use crate::lifecycle::BackgroundRun;
use crate::progress::{ProgressStore};
use crate::dashboard::RunDashboard;

#[test]
fn dashboard_data_accurate_running() {
    let mut run = BackgroundRun::new("r1", 0);
    run.start();
    run.apply_effect("e1");
    let d = RunDashboard::from(&run, None);
    assert_eq!(d.state_label, "running");
    assert_eq!(d.effects_count, 1);
}

#[test]
fn dashboard_data_accurate_completed_with_progress() {
    let mut run = BackgroundRun::new("r1", 0);
    run.start();
    run.complete();
    let mut store = ProgressStore::default();
    store.init("r1", 10);
    for i in 0..10 { store.advance("r1", i); }
    let p = store.get("r1").unwrap();
    let d = RunDashboard::from(&run, Some(p));
    assert_eq!(d.state_label, "completed");
    assert_eq!(d.pct_complete, 100.0);
}

#[test]
fn dashboard_state_label_sleeping() {
    let mut run = BackgroundRun::new("r1", 0);
    run.start();
    run.sleep_until(99);
    let d = RunDashboard::from(&run, None);
    assert_eq!(d.state_label, "sleeping");
}
