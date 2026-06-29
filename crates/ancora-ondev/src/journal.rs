//! Embedded SQLite-compatible journal for on-device agent state.
//!
//! Uses a pure-Rust in-memory store (no network, no external process)
//! that mirrors the SQLite on-disk schema so the same code works in
//! tests and on real devices with an actual SQLite library linked.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A single journal entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalEntry {
    /// Monotonically increasing entry ID.
    pub id: u64,
    /// Agent identifier that produced this entry.
    pub agent_id: String,
    /// Sequence number within the agent's run.
    pub seq: u64,
    /// ISO-8601 timestamp (wall clock, set by the device).
    pub timestamp: String,
    /// Arbitrary payload serialised as JSON.
    pub payload: serde_json::Value,
}

/// Result of a journal write.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WriteResult {
    /// Entry was appended; contains the new row ID.
    Ok(u64),
    /// Write was rejected (e.g., duplicate seq number).
    Rejected(String),
}

/// DDL schema string (mirrors what would be executed against SQLite).
pub const SCHEMA_SQL: &str = "
CREATE TABLE IF NOT EXISTS journal (
    id        INTEGER PRIMARY KEY AUTOINCREMENT,
    agent_id  TEXT    NOT NULL,
    seq       INTEGER NOT NULL,
    timestamp TEXT    NOT NULL,
    payload   TEXT    NOT NULL,
    UNIQUE (agent_id, seq)
);
CREATE INDEX IF NOT EXISTS idx_journal_agent ON journal (agent_id);
";

/// Embedded journal store backed by an in-process `HashMap`.
///
/// On real devices this would delegate to the system SQLite via the
/// `rusqlite` crate; here we use a pure-Rust stand-in so the crate
/// compiles offline on all targets.
#[derive(Debug, Default)]
pub struct Journal {
    next_id: u64,
    rows: Vec<JournalEntry>,
    /// Tracks (agent_id, seq) pairs to enforce the UNIQUE constraint.
    seen: HashMap<(String, u64), ()>,
}

impl Journal {
    /// Create an empty journal (analogous to opening a fresh SQLite file).
    pub fn open() -> Self {
        Self::default()
    }

    /// Append an entry to the journal.
    pub fn append(
        &mut self,
        agent_id: &str,
        seq: u64,
        timestamp: &str,
        payload: serde_json::Value,
    ) -> WriteResult {
        let key = (agent_id.to_string(), seq);
        if self.seen.contains_key(&key) {
            return WriteResult::Rejected(format!(
                "duplicate (agent_id={}, seq={}) ",
                agent_id, seq
            ));
        }
        self.next_id += 1;
        let id = self.next_id;
        self.seen.insert(key, ());
        self.rows.push(JournalEntry {
            id,
            agent_id: agent_id.to_string(),
            seq,
            timestamp: timestamp.to_string(),
            payload,
        });
        WriteResult::Ok(id)
    }

    /// Return all entries for a given agent, ordered by `seq`.
    pub fn entries_for(&self, agent_id: &str) -> Vec<&JournalEntry> {
        let mut v: Vec<&JournalEntry> =
            self.rows.iter().filter(|e| e.agent_id == agent_id).collect();
        v.sort_by_key(|e| e.seq);
        v
    }

    /// Total number of entries across all agents.
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    /// Returns `true` when the journal contains no entries.
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// Delete all entries for an agent (e.g., after a successful upload).
    pub fn purge_agent(&mut self, agent_id: &str) -> usize {
        let before = self.rows.len();
        self.rows.retain(|e| e.agent_id != agent_id);
        self.seen.retain(|(aid, _), _| aid != agent_id);
        before - self.rows.len()
    }

    /// Export the full journal as a JSON array string.
    pub fn export_json(&self) -> String {
        serde_json::to_string(&self.rows).unwrap_or_else(|_| "[]".to_string())
    }
}

#[cfg(test)]
mod unit {
    use super::*;
    use serde_json::json;

    #[test]
    fn append_and_retrieve() {
        let mut j = Journal::open();
        let r = j.append("agent-1", 1, "2026-01-01T00:00:00Z", json!({"step": "init"}));
        assert!(matches!(r, WriteResult::Ok(_)));
        let entries = j.entries_for("agent-1");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].seq, 1);
    }

    #[test]
    fn duplicate_seq_rejected() {
        let mut j = Journal::open();
        j.append("agent-1", 1, "t1", json!(null));
        let r = j.append("agent-1", 1, "t2", json!(null));
        assert!(matches!(r, WriteResult::Rejected(_)));
    }

    #[test]
    fn entries_ordered_by_seq() {
        let mut j = Journal::open();
        j.append("a", 3, "t3", json!({"n": 3}));
        j.append("a", 1, "t1", json!({"n": 1}));
        j.append("a", 2, "t2", json!({"n": 2}));
        let entries = j.entries_for("a");
        assert_eq!(entries[0].seq, 1);
        assert_eq!(entries[2].seq, 3);
    }

    #[test]
    fn purge_removes_correct_agent() {
        let mut j = Journal::open();
        j.append("a", 1, "t", json!(null));
        j.append("b", 1, "t", json!(null));
        let removed = j.purge_agent("a");
        assert_eq!(removed, 1);
        assert_eq!(j.len(), 1);
    }

    #[test]
    fn export_json_is_valid() {
        let mut j = Journal::open();
        j.append("x", 1, "t", json!({"k": "v"}));
        let json_str = j.export_json();
        let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        assert!(parsed.is_array());
    }
}
