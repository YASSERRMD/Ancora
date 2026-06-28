use serde::{Deserialize, Serialize};

/// A single journal entry that can be backed up and replayed.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct JournalEntry {
    pub seq: u64,
    pub run_id: String,
    pub tenant_id: String,
    pub kind: String,
    pub payload: String,
}

/// In-memory journal used as the source for backup and restore.
#[derive(Default, Clone)]
pub struct Journal {
    entries: Vec<JournalEntry>,
}

impl Journal {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn append(&mut self, entry: JournalEntry) {
        self.entries.push(entry);
    }

    /// Export all entries as a full snapshot.
    pub fn snapshot(&self) -> Vec<JournalEntry> {
        self.entries.clone()
    }

    /// Export entries with seq > `since` (incremental export).
    pub fn incremental(&self, since: u64) -> Vec<JournalEntry> {
        self.entries.iter().filter(|e| e.seq > since).cloned().collect()
    }

    /// Export entries up to and including `up_to_seq` (point-in-time).
    pub fn up_to(&self, up_to_seq: u64) -> Vec<JournalEntry> {
        self.entries.iter().filter(|e| e.seq <= up_to_seq).cloned().collect()
    }

    /// Restore from a list of entries, replacing current state.
    pub fn restore(&mut self, entries: Vec<JournalEntry>) {
        self.entries = entries;
    }

    pub fn entries(&self) -> &[JournalEntry] {
        &self.entries
    }

    pub fn max_seq(&self) -> u64 {
        self.entries.iter().map(|e| e.seq).max().unwrap_or(0)
    }
}
