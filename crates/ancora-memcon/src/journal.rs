/// A single journaled consolidation event for replay.
#[derive(Debug, Clone)]
pub struct JournalEntry {
    pub tick: u64,
    pub event: ConsolidationEvent,
}

#[derive(Debug, Clone)]
pub enum ConsolidationEvent {
    Summarized {
        dropped_count: usize,
        summary_len: usize,
    },
    Promoted {
        key: String,
    },
    Forgot {
        key: String,
    },
    Deduped {
        removed_count: usize,
    },
}

/// Append-only journal of consolidation events.
#[derive(Debug, Default)]
pub struct ConsolidationJournal {
    entries: Vec<JournalEntry>,
}

impl ConsolidationJournal {
    pub fn record(&mut self, tick: u64, event: ConsolidationEvent) {
        self.entries.push(JournalEntry { tick, event });
    }

    pub fn entries(&self) -> &[JournalEntry] {
        &self.entries
    }

    pub fn replay_events(&self) -> Vec<&ConsolidationEvent> {
        self.entries.iter().map(|e| &e.event).collect()
    }
}
