use ancora_lh::{BackgroundRun, CheckpointCadence, ScheduledWakeup};

fn build_run(tick: u64) -> BackgroundRun {
    let mut run = BackgroundRun::new("r1", tick);
    run.start();
    run.apply_effect("fetch-data");
    run.apply_effect("fetch-data"); // idempotent duplicate
    run.apply_effect("process");
    run
}

#[test]
fn lh_run_effects_idempotent_stable() {
    let run1 = build_run(1);
    let run2 = build_run(1);
    assert_eq!(run1.effects_applied, run2.effects_applied);
    assert_eq!(run1.effects_applied.len(), 2); // "fetch-data" deduped, "process" kept
}

#[test]
fn lh_wakeup_fires_at_same_tick() {
    let w1 = ScheduledWakeup {
        run_id: "r1".into(),
        wake_at_tick: 100,
    };
    let w2 = ScheduledWakeup {
        run_id: "r1".into(),
        wake_at_tick: 100,
    };
    assert_eq!(w1.should_fire(99), w2.should_fire(99));
    assert_eq!(w1.should_fire(100), w2.should_fire(100));
    assert!(!w1.should_fire(99));
    assert!(w1.should_fire(100));
}

#[test]
fn lh_checkpoint_cadence_stable() {
    let mut c1 = CheckpointCadence::new(10);
    let mut c2 = CheckpointCadence::new(10);
    // Both cadences should fire at the same ticks
    for tick in 0..=30_u64 {
        assert_eq!(c1.should_checkpoint(tick), c2.should_checkpoint(tick));
    }
}
