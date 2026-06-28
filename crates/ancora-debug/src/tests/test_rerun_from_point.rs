/// test_rerun_from_point.rs - Verify re-run-from-a-point semantics via branch + replay.
///
/// "Re-run from a point" means: take a branch at seq N, add edited entries,
/// then replay the resulting journal from seq 0.  The first N entries should
/// match the original; entries after N reflect the new inputs.

use crate::branch::Branch;
use crate::loader::{load_journal, EntryKind, JournalEntry, RunId, Seq};
use crate::replay::Replayer;

fn sc(run: &str, seq: u64, from: &str, to: &str) -> JournalEntry {
    JournalEntry::new(
        RunId::new(run),
        seq,
        EntryKind::StateChange { from: from.into(), to: to.into() },
    )
}

fn original() -> crate::loader::Journal {
    load_journal(vec![
        sc("orig", 0, "boot", "idle"),
        sc("orig", 1, "idle", "planning"),
        sc("orig", 2, "planning", "executing"),
        sc("orig", 3, "executing", "done"),
    ])
    .unwrap()
}

#[test]
fn rerun_prefix_matches_original() {
    let j = original();
    // Branch at seq 1 - rerun from seq 2 onward.
    let mut b = Branch::new("rerun", &j, Seq(1)).unwrap();
    b.append(sc("orig", 99, "planning", "alt-executing")).unwrap();
    b.append(sc("orig", 99, "alt-executing", "alt-done")).unwrap();

    let new_j = b.to_journal(RunId::new("rerun-run")).unwrap();
    let mut r = Replayer::new(&new_j);

    // Steps 0 and 1 should match the original.
    r.step_forward();
    r.step_forward();
    let visited = r.visited();
    assert_eq!(visited.len(), 2);
    if let EntryKind::StateChange { to, .. } = &visited[0].kind {
        assert_eq!(to, "idle");
    }
    if let EntryKind::StateChange { to, .. } = &visited[1].kind {
        assert_eq!(to, "planning");
    }
}

#[test]
fn rerun_extension_reflects_new_entries() {
    let j = original();
    let mut b = Branch::new("rerun2", &j, Seq(1)).unwrap();
    b.append(sc("orig", 99, "planning", "NEW-STATE")).unwrap();

    let new_j = b.to_journal(RunId::new("rerun2-run")).unwrap();
    // The last entry should be our new state.
    let last = new_j.entries().last().unwrap();
    if let EntryKind::StateChange { to, .. } = &last.kind {
        assert_eq!(to, "NEW-STATE");
    }
}

#[test]
fn rerun_journal_length_is_prefix_plus_extension() {
    let j = original();
    // Branch at seq 2 (prefix = 3 entries: 0, 1, 2).
    let mut b = Branch::new("rerun3", &j, Seq(2)).unwrap();
    b.append(sc("orig", 99, "executing", "fast-done")).unwrap();
    // Extension = 1 entry; total = 4.
    let new_j = b.to_journal(RunId::new("rerun3-run")).unwrap();
    assert_eq!(new_j.len(), 4);
}

#[test]
fn rerun_from_start_replays_entire_alternate() {
    let j = original();
    let mut b = Branch::new("rerun4", &j, Seq(0)).unwrap();
    b.append(sc("orig", 99, "boot", "skipped-idle")).unwrap();
    b.append(sc("orig", 99, "skipped-idle", "done-fast")).unwrap();

    let new_j = b.to_journal(RunId::new("rerun4-run")).unwrap();
    let mut r = Replayer::new(&new_j);
    let mut states = Vec::new();
    r.run_to_end(|e| {
        if let EntryKind::StateChange { to, .. } = &e.kind {
            states.push(to.clone());
        }
    });
    // First entry matches original branch point ("boot" -> "idle"), then our two new ones.
    assert!(states.contains(&"skipped-idle".to_string()));
    assert!(states.contains(&"done-fast".to_string()));
}
