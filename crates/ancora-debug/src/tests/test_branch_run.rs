/// test_branch_run.rs - Verify that branching creates a valid alternate run.

use crate::branch::{Branch, BranchError, BranchRegistry};
use crate::loader::{load_journal, EntryKind, JournalEntry, RunId, Seq};

fn sc(run: &str, seq: u64, from: &str, to: &str) -> JournalEntry {
    JournalEntry::new(
        RunId::new(run),
        seq,
        EntryKind::StateChange { from: from.into(), to: to.into() },
    )
}

fn sample() -> crate::loader::Journal {
    load_journal(vec![
        sc("orig", 0, "init", "a"),
        sc("orig", 1, "a", "b"),
        sc("orig", 2, "b", "c"),
        sc("orig", 3, "c", "done"),
    ])
    .unwrap()
}

#[test]
fn branch_prefix_length_matches_branch_point() {
    let j = sample();
    let b = Branch::new("b1", &j, Seq(2)).unwrap();
    // Prefix should include entries 0, 1, 2 (inclusive).
    assert_eq!(b.prefix_entries().len(), 3);
    assert_eq!(b.extension_entries().len(), 0);
}

#[test]
fn branch_total_len_after_extension() {
    let j = sample();
    let mut b = Branch::new("b1", &j, Seq(1)).unwrap();
    b.append(sc("orig", 99, "a", "alt-c")).unwrap();
    b.append(sc("orig", 99, "alt-c", "alt-done")).unwrap();
    assert_eq!(b.len(), 4); // 2 prefix + 2 extension
}

#[test]
fn branch_materialised_journal_has_contiguous_seqs() {
    let j = sample();
    let mut b = Branch::new("b1", &j, Seq(2)).unwrap();
    b.append(sc("orig", 99, "b", "alt-done")).unwrap();
    let new_j = b.to_journal(RunId::new("branch-run")).unwrap();
    // Should have 4 entries with seqs 0..3.
    assert_eq!(new_j.len(), 4);
    for (i, e) in new_j.entries().iter().enumerate() {
        assert_eq!(e.seq.0, i as u64);
    }
}

#[test]
fn branch_out_of_range_returns_error() {
    let j = sample();
    let result = Branch::new("b1", &j, Seq(100));
    assert!(matches!(result, Err(BranchError::SeqOutOfRange { .. })));
}

#[test]
fn branch_registry_manages_multiple_branches() {
    let j = sample();
    let b1 = Branch::new("b1", &j, Seq(0)).unwrap();
    let b2 = Branch::new("b2", &j, Seq(2)).unwrap();
    let mut reg = BranchRegistry::new();
    reg.insert(b1);
    reg.insert(b2);
    assert_eq!(reg.count(), 2);
    assert!(reg.get("b1").is_some());
    assert!(reg.get("b2").is_some());
    assert!(reg.get("b3").is_none());
}

#[test]
fn branch_entries_iterator_covers_all() {
    let j = sample();
    let mut b = Branch::new("b1", &j, Seq(1)).unwrap();
    b.append(sc("orig", 99, "a", "alt")).unwrap();
    let count = b.entries().count();
    assert_eq!(count, 3); // 2 prefix + 1 extension
}
