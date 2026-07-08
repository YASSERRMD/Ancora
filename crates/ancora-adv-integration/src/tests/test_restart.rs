use ancora_coord::CoordJournal;
use ancora_lh::{BackgroundRun, Checkpoint};

#[test]
fn combined_runs_survive_restart() {
    // Simulate a run that is checkpointed, then restarted
    let mut run = BackgroundRun::new("run-restart", 1);
    run.start();
    run.apply_effect("effect-1");
    run.apply_effect("effect-2");

    // Save checkpoint
    let mut ck = Checkpoint::new("run-restart", 1);
    ck.set("effects_count", &run.effects_applied.len().to_string());

    // Simulate restart: rebuild run from checkpoint
    let mut run2 = BackgroundRun::new("run-restart", 1);
    run2.start();
    let count_str = ck.get("effects_count").unwrap_or("0");
    let count: usize = count_str.parse().unwrap();
    assert_eq!(count, 2);

    // Replay effects idempotently
    run2.apply_effect("effect-1");
    run2.apply_effect("effect-1"); // idempotent, should not duplicate
    assert_eq!(run2.effects_applied.len(), 1);
}

#[test]
fn coord_journal_survives_restart_replay() {
    let mut journal = CoordJournal::default();
    journal.record(1, "blackboard", "wrote key=task");
    journal.record(2, "auction", "bid submitted");

    // Replay produces same sequence
    let replayed = journal.replay();
    assert_eq!(replayed.len(), 2);
    assert_eq!(replayed[0].0, "blackboard");
    assert_eq!(replayed[1].0, "auction");
}
