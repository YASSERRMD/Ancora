/// test_run_diff.rs - Verify the diff engine correctly identifies divergence.
use crate::diff::{diff_journals, EntryDiff};
use crate::loader::{load_journal, EntryKind, JournalEntry, RunId, Seq};

fn sc(run: &str, seq: u64, from: &str, to: &str) -> JournalEntry {
    JournalEntry::new(
        RunId::new(run),
        seq,
        EntryKind::StateChange {
            from: from.into(),
            to: to.into(),
        },
    )
}

fn llm(run: &str, seq: u64, prompt: &str, response: &str) -> JournalEntry {
    JournalEntry::new(
        RunId::new(run),
        seq,
        EntryKind::LlmExchange {
            prompt: prompt.into(),
            response: response.into(),
        },
    )
}

#[test]
fn diff_identical_returns_no_divergence() {
    let j1 = load_journal(vec![
        sc("r1", 0, "a", "b"),
        sc("r1", 1, "b", "c"),
        sc("r1", 2, "c", "d"),
    ])
    .unwrap();
    let j2 = load_journal(vec![
        sc("r2", 0, "a", "b"),
        sc("r2", 1, "b", "c"),
        sc("r2", 2, "c", "d"),
    ])
    .unwrap();
    let diff = diff_journals(&j1, &j2);
    assert!(diff.is_identical());
    assert!(diff.first_divergence.is_none());
}

#[test]
fn diff_highlights_first_divergence_seq() {
    let j1 = load_journal(vec![sc("r1", 0, "a", "b"), sc("r1", 1, "b", "c")]).unwrap();
    let j2 = load_journal(vec![sc("r2", 0, "a", "b"), sc("r2", 1, "b", "DIFFERENT")]).unwrap();
    let diff = diff_journals(&j1, &j2);
    assert_eq!(diff.first_divergence, Some(Seq(1)));
}

#[test]
fn diff_kind_changed_reported() {
    let j1 = load_journal(vec![sc("r1", 0, "a", "b")]).unwrap();
    let j2 = load_journal(vec![llm("r2", 0, "prompt", "response")]).unwrap();
    let diff = diff_journals(&j1, &j2);
    assert!(matches!(diff.lines[0].diff, EntryDiff::KindChanged { .. }));
}

#[test]
fn diff_only_in_left_when_right_is_shorter() {
    let j1 = load_journal(vec![sc("r1", 0, "a", "b"), sc("r1", 1, "b", "c")]).unwrap();
    let j2 = load_journal(vec![sc("r2", 0, "a", "b")]).unwrap();
    let diff = diff_journals(&j1, &j2);
    assert!(diff.lines.iter().any(|l| l.diff == EntryDiff::OnlyInLeft));
}

#[test]
fn diff_only_in_right_when_left_is_shorter() {
    let j1 = load_journal(vec![sc("r1", 0, "a", "b")]).unwrap();
    let j2 = load_journal(vec![sc("r2", 0, "a", "b"), sc("r2", 1, "b", "c")]).unwrap();
    let diff = diff_journals(&j1, &j2);
    assert!(diff.lines.iter().any(|l| l.diff == EntryDiff::OnlyInRight));
}

#[test]
fn changed_lines_excludes_equal_entries() {
    let j1 = load_journal(vec![
        sc("r1", 0, "a", "b"),
        sc("r1", 1, "b", "c"),
        sc("r1", 2, "c", "d"),
    ])
    .unwrap();
    let j2 = load_journal(vec![
        sc("r2", 0, "a", "b"),
        sc("r2", 1, "b", "DIFF"),
        sc("r2", 2, "c", "d"),
    ])
    .unwrap();
    let diff = diff_journals(&j1, &j2);
    let changed = diff.changed_lines();
    assert_eq!(changed.len(), 1);
    assert_eq!(changed[0].seq, Seq(1));
}
