// Chaos: partial journal write -- replay detects truncated last entry.

struct PartialJournal {
    entries: Vec<String>,
    truncated: bool,
}

impl PartialJournal {
    fn new(entries: Vec<String>, truncated: bool) -> Self { Self { entries, truncated } }

    fn is_complete(&self) -> bool {
        !self.truncated && self.entries.last().map(|e| e == "completed").unwrap_or(false)
    }

    fn recoverable_count(&self) -> usize {
        let last = if self.truncated { self.entries.len().saturating_sub(1) } else { self.entries.len() };
        last
    }
}

#[test]
fn test_complete_journal_is_complete() {
    let j = PartialJournal::new(vec!["started".into(), "activity".into(), "completed".into()], false);
    assert!(j.is_complete());
}

#[test]
fn test_truncated_journal_is_not_complete() {
    let j = PartialJournal::new(vec!["started".into(), "activity".into()], true);
    assert!(!j.is_complete());
}

#[test]
fn test_missing_completed_event_is_not_complete() {
    let j = PartialJournal::new(vec!["started".into(), "activity".into()], false);
    assert!(!j.is_complete());
}

#[test]
fn test_truncated_entry_excluded_from_recovery() {
    let j = PartialJournal::new(vec!["a".into(), "b".into(), "partial_c".into()], true);
    assert_eq!(j.recoverable_count(), 2);
}

#[test]
fn test_non_truncated_all_entries_recoverable() {
    let j = PartialJournal::new(vec!["a".into(), "b".into(), "completed".into()], false);
    assert_eq!(j.recoverable_count(), 3);
}

#[test]
fn test_empty_truncated_journal_recovers_zero() {
    let j = PartialJournal::new(vec![], true);
    assert_eq!(j.recoverable_count(), 0);
}
