/// loader.rs - Load a run journal for offline inspection.
///
/// A run journal is a sequence of [`JournalEntry`] records that capture
/// every observable event in an agent run: state transitions, LLM
/// prompt/response pairs, and tool invocations.  The loader reads a
/// serialised journal (here represented as a slice of pre-parsed entries)
/// and validates its internal consistency before handing it to the rest
/// of the debug toolkit.

use std::collections::HashMap;

/// Opaque identifier for a single agent run.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RunId(pub String);

impl RunId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

/// A monotonically increasing sequence number within a run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Seq(pub u64);

/// The payload carried by each journal entry.
#[derive(Debug, Clone)]
pub enum EntryKind {
    /// Agent transitioned to a new named state.
    StateChange { from: String, to: String },
    /// An LLM prompt was sent and a response received.
    LlmExchange { prompt: String, response: String },
    /// A tool was called with the given arguments and produced the given output.
    ToolCall {
        tool_name: String,
        args: HashMap<String, String>,
        output: String,
    },
    /// A free-form annotation added by a developer.
    Annotation { text: String },
}

/// One record in the journal.
#[derive(Debug, Clone)]
pub struct JournalEntry {
    pub run_id: RunId,
    pub seq: Seq,
    pub kind: EntryKind,
}

impl JournalEntry {
    pub fn new(run_id: RunId, seq: u64, kind: EntryKind) -> Self {
        Self { run_id, seq: Seq(seq), kind }
    }
}

/// A fully-loaded run journal ready for debugging.
#[derive(Debug, Clone)]
pub struct Journal {
    pub run_id: RunId,
    entries: Vec<JournalEntry>,
}

/// Errors that can occur while loading a journal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadError {
    /// The journal is empty.
    Empty,
    /// Two entries share the same sequence number.
    DuplicateSeq(u64),
    /// Entries belong to more than one run.
    MixedRunIds,
    /// The sequence numbers contain a gap.
    SeqGap { expected: u64, found: u64 },
}

impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadError::Empty => write!(f, "journal is empty"),
            LoadError::DuplicateSeq(s) => write!(f, "duplicate sequence number {}", s),
            LoadError::MixedRunIds => write!(f, "entries belong to multiple run ids"),
            LoadError::SeqGap { expected, found } => {
                write!(f, "sequence gap: expected {} found {}", expected, found)
            }
        }
    }
}

impl std::error::Error for LoadError {}

/// Load and validate a journal from a slice of pre-parsed entries.
///
/// The entries may be provided in any order; this function sorts them by
/// sequence number and validates consistency.
pub fn load_journal(mut entries: Vec<JournalEntry>) -> Result<Journal, LoadError> {
    if entries.is_empty() {
        return Err(LoadError::Empty);
    }

    // All entries must belong to the same run.
    let run_id = entries[0].run_id.clone();
    if entries.iter().any(|e| e.run_id != run_id) {
        return Err(LoadError::MixedRunIds);
    }

    // Sort by sequence number.
    entries.sort_by_key(|e| e.seq);

    // Validate sequence numbers are contiguous starting from 0.
    for (i, entry) in entries.iter().enumerate() {
        let expected = i as u64;
        if entry.seq.0 != expected {
            if entries.iter().filter(|e| e.seq == entry.seq).count() > 1 {
                return Err(LoadError::DuplicateSeq(entry.seq.0));
            }
            return Err(LoadError::SeqGap { expected, found: entry.seq.0 });
        }
    }

    Ok(Journal { run_id, entries })
}

impl Journal {
    /// Return all entries in sequence order.
    pub fn entries(&self) -> &[JournalEntry] {
        &self.entries
    }

    /// Return the entry at the given sequence number, if it exists.
    pub fn entry_at(&self, seq: Seq) -> Option<&JournalEntry> {
        self.entries.get(seq.0 as usize)
    }

    /// Total number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// True when the journal contains no entries.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_entry(seq: u64) -> JournalEntry {
        JournalEntry::new(
            RunId::new("run-1"),
            seq,
            EntryKind::StateChange {
                from: "idle".into(),
                to: "running".into(),
            },
        )
    }

    #[test]
    fn load_empty_returns_error() {
        assert_eq!(load_journal(vec![]).unwrap_err(), LoadError::Empty);
    }

    #[test]
    fn load_valid_journal() {
        let entries = vec![make_entry(0), make_entry(1), make_entry(2)];
        let journal = load_journal(entries).unwrap();
        assert_eq!(journal.len(), 3);
    }

    #[test]
    fn load_out_of_order_is_sorted() {
        let entries = vec![make_entry(2), make_entry(0), make_entry(1)];
        let journal = load_journal(entries).unwrap();
        assert_eq!(journal.entry_at(Seq(0)).unwrap().seq.0, 0);
    }

    #[test]
    fn load_detects_gap() {
        let entries = vec![make_entry(0), make_entry(2)];
        assert!(matches!(
            load_journal(entries),
            Err(LoadError::SeqGap { expected: 1, found: 2 })
        ));
    }

    #[test]
    fn load_detects_mixed_run_ids() {
        let mut e2 = make_entry(1);
        e2.run_id = RunId::new("run-2");
        let result = load_journal(vec![make_entry(0), e2]);
        assert_eq!(result.unwrap_err(), LoadError::MixedRunIds);
    }
}
