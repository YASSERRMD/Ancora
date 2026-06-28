// Simulates a "restart" by serializing state to primitive types and
// reconstructing identical structs from those values, then verifying results match.
use ancora_coord::CoordJournal;
use ancora_lh::BackgroundRun;
use ancora_reason::ReasoningJournal;

fn saved_effects() -> Vec<String> {
    vec!["init".into(), "fetch".into(), "process".into()]
}

fn restore_run(effects: &[String]) -> BackgroundRun {
    let mut run = BackgroundRun::new("run-restart", 1);
    run.start();
    for e in effects {
        run.apply_effect(e);
    }
    run
}

#[test]
fn restart_run_effects_match_original() {
    let mut original = BackgroundRun::new("run-restart", 1);
    original.start();
    for e in saved_effects() {
        original.apply_effect(&e);
    }

    let restored = restore_run(&saved_effects());
    assert_eq!(original.effects_applied, restored.effects_applied);
}

#[test]
fn restart_coord_journal_replay_matches() {
    let mut j1 = CoordJournal::default();
    j1.record(1, "assign", "t1 -> a1");
    j1.record(2, "complete", "t1 done");

    // Simulate restart: replay j1 into j2
    let events: Vec<(u64, String, String)> = j1
        .events()
        .iter()
        .map(|e| (e.tick, e.kind.clone(), e.description.clone()))
        .collect();

    let mut j2 = CoordJournal::default();
    for (tick, kind, desc) in &events {
        j2.record(*tick, kind, desc);
    }

    assert_eq!(j1.replay(), j2.replay());
}

#[test]
fn restart_reasoning_journal_event_count_matches() {
    let mut j1 = ReasoningJournal::default();
    j1.record(1, ancora_reason::ReasoningEvent::StepVerified { index: 0 });
    j1.record(2, ancora_reason::ReasoningEvent::StepRefuted { index: 1 });

    let count_before = j1.events().len();

    // Simulate restart by replaying all events into a fresh journal
    let mut j2 = ReasoningJournal::default();
    for (tick, ev) in j1.events() {
        j2.record(*tick, ev.clone());
    }

    assert_eq!(count_before, j2.events().len());
}
