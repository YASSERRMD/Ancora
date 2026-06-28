use crate::journal::CoordJournal;

#[test]
fn journal_records_coordination_event() {
    let mut j = CoordJournal::default();
    j.record(1, "bid", "agent-a submitted bid 0.9");
    assert_eq!(j.events().len(), 1);
}

#[test]
fn coordination_replays_deterministically() {
    let mut j = CoordJournal::default();
    j.record(1, "assign", "agent-b won task-1");
    j.record(2, "conflict", "resolved by priority");
    let replayed = j.replay();
    assert_eq!(replayed[0], ("assign", "agent-b won task-1"));
    assert_eq!(replayed[1], ("conflict", "resolved by priority"));
}
