/// Swap journal: append-only log of swap and rollback events for replay.
use crate::model::ModelVersion;

/// The kind of event recorded in the journal.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum SwapEvent {
    /// A model was swapped from one version to another.
    Swap {
        from: ModelVersion,
        to: ModelVersion,
    },
    /// A rollback was performed, restoring a prior version.
    Rollback {
        from: ModelVersion,
        to: ModelVersion,
    },
}

/// A single entry in the swap journal.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JournalEntry {
    /// The swap or rollback that occurred.
    pub event: SwapEvent,
    /// Wall-clock timestamp in nanoseconds since the runtime started
    /// (used for ordering and replay).
    pub timestamp_ns: u64,
}

/// An append-only, cloneable swap journal.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct SwapJournal {
    entries: Vec<JournalEntry>,
}

impl SwapJournal {
    /// Create an empty journal.
    pub fn new() -> Self {
        SwapJournal::default()
    }

    /// Append a new entry.
    pub fn append(&mut self, entry: JournalEntry) {
        self.entries.push(entry);
    }

    /// All entries in order.
    pub fn entries(&self) -> &[JournalEntry] {
        &self.entries
    }

    /// Number of events recorded.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// True if no events have been recorded yet.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Serialize the journal to JSON for persistence / replay.
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("journal serialization cannot fail")
    }

    /// Deserialize a journal from JSON.
    pub fn from_json(s: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(s)
    }

    /// Reconstruct the sequence of (from, to) version pairs for replay
    /// inspection without re-running the actual swaps.
    pub fn replay_sequence(&self) -> Vec<(ModelVersion, ModelVersion)> {
        self.entries
            .iter()
            .map(|e| match &e.event {
                SwapEvent::Swap { from, to } => (*from, *to),
                SwapEvent::Rollback { from, to } => (*from, *to),
            })
            .collect()
    }
}
