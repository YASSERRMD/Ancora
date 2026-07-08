use ancora_lh::{BackgroundRun, Checkpoint, Deadline, ScheduledWakeup, Throttle};

#[test]
fn lh_parity_run_lifecycle() {
    let mut run = BackgroundRun::new("parity-run", 1);
    run.start();
    run.apply_effect("step-1");
    run.apply_effect("step-2");
    run.apply_effect("step-1"); // idempotent
    assert_eq!(run.effects_applied.len(), 2);
    run.complete();
}

#[test]
fn lh_parity_scheduled_wakeup() {
    let w = ScheduledWakeup::new("r", 50);
    assert!(!w.should_fire(49));
    assert!(w.should_fire(50));
    assert!(w.should_fire(51));
}

#[test]
fn lh_parity_checkpoint_data() {
    let mut ck = Checkpoint::new("r", 10);
    ck.set("phase", "init");
    ck.set("step", "3");
    assert_eq!(ck.get("phase"), Some("init"));
    assert_eq!(ck.get("step"), Some("3"));
    assert_eq!(ck.get("missing"), None);
}

#[test]
fn lh_parity_deadline_exceeded() {
    let d = Deadline {
        run_id: "r".into(),
        deadline_tick: 100,
    };
    assert!(d.check(101).is_err());
    assert!(d.check(100).is_ok());
}

#[test]
fn lh_parity_throttle_cap() {
    let mut t = Throttle::new(3);
    assert!(t.try_op(1).is_ok());
    assert!(t.try_op(1).is_ok());
    assert!(t.try_op(1).is_ok());
    assert!(t.try_op(1).is_err()); // throttled
}
