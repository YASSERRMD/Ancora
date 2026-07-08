use serde::{Deserialize, Serialize};

/// A replicated journal entry used by both primary and secondary stores.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JournalEntry {
    pub seq: u64,
    pub data: String,
}

/// Simulated journal store (primary or secondary).
#[derive(Default, Clone, Debug)]
pub struct JournalStore {
    pub entries: Vec<JournalEntry>,
    pub fenced: bool,
}

impl JournalStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn append(&mut self, entry: JournalEntry) -> Result<(), &'static str> {
        if self.fenced {
            return Err("store is fenced");
        }
        self.entries.push(entry);
        Ok(())
    }

    pub fn max_seq(&self) -> u64 {
        self.entries.iter().map(|e| e.seq).max().unwrap_or(0)
    }

    pub fn fence(&mut self) {
        self.fenced = true;
    }

    pub fn unfence(&mut self) {
        self.fenced = false;
    }

    pub fn entries_since(&self, seq: u64) -> Vec<JournalEntry> {
        self.entries
            .iter()
            .filter(|e| e.seq > seq)
            .cloned()
            .collect()
    }
}
