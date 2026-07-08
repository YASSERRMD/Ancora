use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use ancora_proto::ancora::JournalEvent;

use crate::error::AncoraError;

/// Durable, ordered storage for journal events.
///
/// Implementations must guarantee:
/// - Events returned by `read` are ordered by `seq` ascending.
/// - `append` is atomic per event: partial writes are not visible.
/// - Duplicate `activity_key` on `ActivityRecorded` events must be rejected
///   with `AncoraError::JournalWrite`.
pub trait JournalStore: Send + Sync {
    /// Append a single event. Returns the assigned sequence number.
    fn append(&self, run_id: &str, event: JournalEvent) -> Result<u64, AncoraError>;

    /// Read all events for a run in seq-ascending order.
    fn read(&self, run_id: &str) -> Result<Vec<JournalEvent>, AncoraError>;

    /// Load a single event by run and sequence number.
    fn load(&self, run_id: &str, seq: u64) -> Result<Option<JournalEvent>, AncoraError>;
}

/// Opaque checkpoint blob saved at the end of each node so a run can
/// resume from a known-good point without replaying the full journal.
///
/// The bytes are engine-internal; no schema is imposed by this trait.
pub trait CheckpointStore: Send + Sync {
    /// Persist a checkpoint for a run at the given sequence number.
    /// Overwrites any prior checkpoint for the same run.
    fn save(&self, run_id: &str, at_seq: u64, data: &[u8]) -> Result<(), AncoraError>;

    /// Load the most recent checkpoint for a run.
    /// Returns `None` if no checkpoint has been saved yet.
    fn load_checkpoint(&self, run_id: &str) -> Result<Option<(u64, Vec<u8>)>, AncoraError>;
}

// ---- In-memory implementation ----

#[derive(Default)]
struct MemState {
    /// run_id -> events in seq order
    events: HashMap<String, Vec<JournalEvent>>,
    /// run_id -> (at_seq, bytes)
    checkpoints: HashMap<String, (u64, Vec<u8>)>,
}

/// A non-durable in-memory store used for tests and single-process runs.
#[derive(Clone, Default)]
pub struct MemoryStore {
    state: Arc<Mutex<MemState>>,
}

impl MemoryStore {
    pub fn new() -> Self {
        Self::default()
    }
}

impl JournalStore for MemoryStore {
    fn append(&self, run_id: &str, mut event: JournalEvent) -> Result<u64, AncoraError> {
        let mut state = self
            .state
            .lock()
            .map_err(|_| AncoraError::Storage("mutex poisoned".to_string()))?;
        let events = state.events.entry(run_id.to_string()).or_default();
        let seq = events.len() as u64;
        event.seq = seq;
        event.run_id = run_id.to_string();
        events.push(event);
        Ok(seq)
    }

    fn read(&self, run_id: &str) -> Result<Vec<JournalEvent>, AncoraError> {
        let state = self
            .state
            .lock()
            .map_err(|_| AncoraError::Storage("mutex poisoned".to_string()))?;
        Ok(state.events.get(run_id).cloned().unwrap_or_default())
    }

    fn load(&self, run_id: &str, seq: u64) -> Result<Option<JournalEvent>, AncoraError> {
        let state = self
            .state
            .lock()
            .map_err(|_| AncoraError::Storage("mutex poisoned".to_string()))?;
        Ok(state
            .events
            .get(run_id)
            .and_then(|evs| evs.get(seq as usize))
            .cloned())
    }
}

impl CheckpointStore for MemoryStore {
    fn save(&self, run_id: &str, at_seq: u64, data: &[u8]) -> Result<(), AncoraError> {
        let mut state = self
            .state
            .lock()
            .map_err(|_| AncoraError::Storage("mutex poisoned".to_string()))?;
        state
            .checkpoints
            .insert(run_id.to_string(), (at_seq, data.to_vec()));
        Ok(())
    }

    fn load_checkpoint(&self, run_id: &str) -> Result<Option<(u64, Vec<u8>)>, AncoraError> {
        let state = self
            .state
            .lock()
            .map_err(|_| AncoraError::Storage("mutex poisoned".to_string()))?;
        Ok(state.checkpoints.get(run_id).cloned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ancora_proto::ancora::{journal_event::Event, JournalEvent, RunStartedEvent};

    fn make_event(label: &str) -> JournalEvent {
        JournalEvent {
            event_id: label.to_string(),
            run_id: String::new(),
            seq: 0,
            recorded_at_ns: 0,
            event: Some(Event::RunStarted(RunStartedEvent {
                run_id: label.to_string(),
                spec_bytes: vec![],
                spec_type: "AgentSpec".to_string(),
            })),
        }
    }

    #[test]
    fn append_then_read_returns_ordered_events() {
        let store = MemoryStore::new();
        let seq0 = store.append("run-1", make_event("e0")).unwrap();
        let seq1 = store.append("run-1", make_event("e1")).unwrap();
        let seq2 = store.append("run-1", make_event("e2")).unwrap();

        assert_eq!(seq0, 0);
        assert_eq!(seq1, 1);
        assert_eq!(seq2, 2);

        let events = store.read("run-1").unwrap();
        assert_eq!(events.len(), 3);
        for (i, ev) in events.iter().enumerate() {
            assert_eq!(ev.seq, i as u64, "seq must be monotonically increasing");
        }
    }

    #[test]
    fn load_returns_correct_event_by_seq() {
        let store = MemoryStore::new();
        store.append("run-2", make_event("first")).unwrap();
        store.append("run-2", make_event("second")).unwrap();

        let ev = store.load("run-2", 1).unwrap().expect("seq 1 must exist");
        assert_eq!(ev.seq, 1);

        assert!(store.load("run-2", 99).unwrap().is_none());
    }

    #[test]
    fn read_empty_run_returns_empty_vec() {
        let store = MemoryStore::new();
        assert!(store.read("no-such-run").unwrap().is_empty());
    }

    #[test]
    fn checkpoint_round_trips() {
        let store = MemoryStore::new();
        assert!(store.load_checkpoint("run-3").unwrap().is_none());

        store.save("run-3", 5, b"state-blob").unwrap();
        let (seq, data) = store.load_checkpoint("run-3").unwrap().unwrap();
        assert_eq!(seq, 5);
        assert_eq!(data, b"state-blob");

        store.save("run-3", 10, b"newer-blob").unwrap();
        let (seq2, data2) = store.load_checkpoint("run-3").unwrap().unwrap();
        assert_eq!(seq2, 10);
        assert_eq!(data2, b"newer-blob");
    }

    #[test]
    fn concurrent_appends_preserve_sequence() {
        use std::sync::Arc;
        use std::thread;

        let store = Arc::new(MemoryStore::new());
        let threads: Vec<_> = (0..8)
            .map(|i| {
                let s = Arc::clone(&store);
                thread::spawn(move || s.append("run-conc", make_event(&format!("e{i}"))).unwrap())
            })
            .collect();

        let mut seqs: Vec<u64> = threads.into_iter().map(|h| h.join().unwrap()).collect();
        seqs.sort_unstable();

        assert_eq!(seqs, vec![0, 1, 2, 3, 4, 5, 6, 7]);

        let events = store.read("run-conc").unwrap();
        assert_eq!(events.len(), 8);
        for (i, ev) in events.iter().enumerate() {
            assert_eq!(ev.seq, i as u64);
        }
    }
}
