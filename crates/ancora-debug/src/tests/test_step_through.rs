/// test_step_through.rs - Verify that the replayer reaches every entry.
use crate::loader::{load_journal, EntryKind, JournalEntry, RunId, Seq};
use crate::replay::{Replayer, StepResult};

fn build_journal(n: usize) -> crate::loader::Journal {
    let entries: Vec<JournalEntry> = (0..n as u64)
        .map(|i| {
            JournalEntry::new(
                RunId::new("run-step"),
                i,
                EntryKind::StateChange {
                    from: format!("s{}", i),
                    to: format!("s{}", i + 1),
                },
            )
        })
        .collect();
    load_journal(entries).unwrap()
}

#[test]
fn step_through_visits_every_entry() {
    let j = build_journal(7);
    let mut r = Replayer::new(&j);
    let mut visited = Vec::new();
    while let StepResult::Stepped(e) = r.step_forward() {
        visited.push(e.seq.0);
    }
    assert_eq!(visited, vec![0, 1, 2, 3, 4, 5, 6]);
}

#[test]
fn step_through_forward_then_backward_returns_to_start() {
    let j = build_journal(4);
    let mut r = Replayer::new(&j);
    // go all the way forward
    for _ in 0..4 {
        r.step_forward();
    }
    assert_eq!(r.cursor(), Some(Seq(3)));
    // now walk backward past the start
    for _ in 0..5 {
        r.step_backward();
    }
    assert!(r.cursor().is_none());
}

#[test]
fn step_through_single_entry_journal() {
    let j = build_journal(1);
    let mut r = Replayer::new(&j);
    assert!(matches!(r.step_forward(), StepResult::Stepped(_)));
    assert!(matches!(r.step_forward(), StepResult::AtBoundary));
}

#[test]
fn remaining_decreases_as_we_step() {
    let j = build_journal(5);
    let mut r = Replayer::new(&j);
    assert_eq!(r.remaining().len(), 5);
    r.step_forward();
    assert_eq!(r.remaining().len(), 4);
    r.step_forward();
    assert_eq!(r.remaining().len(), 3);
}
