use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};

use ancora_proto::ancora::{journal_event::Event, ActivityRecordedEvent, JournalEvent};

use crate::error::AncoraError;
use crate::journal::JournalStore;

/// One recorded activity entry in a fixture file.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FixtureEntry {
    pub activity_key: String,
    pub activity_kind: String,
    pub input_json: String,
    pub result_json: String,
}

/// In-memory fixture built from recorded activities.
#[derive(Debug, Clone, Default)]
pub struct Fixture {
    /// activity_key -> recorded result
    entries: Vec<FixtureEntry>,
    index: HashMap<String, String>,
}

impl Fixture {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, entry: FixtureEntry) {
        self.index.insert(entry.activity_key.clone(), entry.result_json.clone());
        self.entries.push(entry);
    }

    /// Look up the recorded result for an activity key.
    pub fn get_result(&self, activity_key: &str) -> Option<&str> {
        self.index.get(activity_key).map(|s| s.as_str())
    }

    pub fn entries(&self) -> &[FixtureEntry] {
        &self.entries
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Merge another fixture into this one, overwriting on key collision.
    pub fn merge(&mut self, other: Fixture) {
        for entry in other.entries {
            self.index.insert(entry.activity_key.clone(), entry.result_json.clone());
            if let Some(existing) = self.entries.iter_mut().find(|e| e.activity_key == entry.activity_key) {
                *existing = entry;
            } else {
                self.entries.push(entry);
            }
        }
    }
}

/// Write fixture entries to a JSONL file (one JSON object per line).
pub fn record_fixture_to_file(fixture: &Fixture, path: &Path) -> Result<(), AncoraError> {
    let mut file = std::fs::File::create(path)
        .map_err(|e| AncoraError::Storage(e.to_string()))?;
    for entry in fixture.entries() {
        let line = serde_json::to_string(entry)
            .map_err(|e| AncoraError::Storage(e.to_string()))?;
        writeln!(file, "{}", line)
            .map_err(|e| AncoraError::Storage(e.to_string()))?;
    }
    Ok(())
}

/// Load fixture entries from a JSONL file.
pub fn load_fixture_from_file(path: &Path) -> Result<Fixture, AncoraError> {
    let file = std::fs::File::open(path)
        .map_err(|e| AncoraError::Storage(e.to_string()))?;
    let reader = BufReader::new(file);
    let mut fixture = Fixture::new();
    for line in reader.lines() {
        let line = line.map_err(|e| AncoraError::Storage(e.to_string()))?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let entry: FixtureEntry = serde_json::from_str(trimmed)
            .map_err(|e| AncoraError::Storage(e.to_string()))?;
        fixture.add(entry);
    }
    Ok(fixture)
}

/// Records activities during a live run to produce a replayable fixture.
pub struct FixtureRecorder {
    fixture: std::sync::Mutex<Fixture>,
}

impl FixtureRecorder {
    pub fn new() -> Self {
        Self { fixture: std::sync::Mutex::new(Fixture::new()) }
    }

    pub fn record(&self, key: &str, kind: &str, input: &str, result: &str) {
        let entry = FixtureEntry {
            activity_key: key.to_string(),
            activity_kind: kind.to_string(),
            input_json: input.to_string(),
            result_json: result.to_string(),
        };
        self.fixture.lock().unwrap().add(entry);
    }

    pub fn into_fixture(self) -> Fixture {
        self.fixture.into_inner().unwrap()
    }

    pub fn snapshot(&self) -> Fixture {
        self.fixture.lock().unwrap().clone()
    }
}

impl Default for FixtureRecorder {
    fn default() -> Self { Self::new() }
}

/// A read-only journal store backed by a fixture.
/// `read` returns ActivityRecorded events for each fixture entry.
pub struct FixtureJournalStore {
    fixture: Fixture,
}

impl FixtureJournalStore {
    pub fn new(fixture: Fixture) -> Self {
        Self { fixture }
    }
}

impl JournalStore for FixtureJournalStore {
    fn append(&self, _run_id: &str, _event: JournalEvent) -> Result<u64, AncoraError> {
        Err(AncoraError::Storage("FixtureJournalStore is read-only".into()))
    }

    fn read(&self, run_id: &str) -> Result<Vec<JournalEvent>, AncoraError> {
        let events = self
            .fixture
            .entries()
            .iter()
            .enumerate()
            .map(|(i, entry)| JournalEvent {
                event_id: format!("fixture-{i}"),
                run_id: run_id.to_string(),
                seq: i as u64,
                recorded_at_ns: 0,
                event: Some(Event::ActivityRecorded(ActivityRecordedEvent {
                    activity_key: entry.activity_key.clone(),
                    activity_kind: entry.activity_kind.clone(),
                    input_json: entry.input_json.clone(),
                    result_json: entry.result_json.clone(),
                    replayed: true,
                })),
            })
            .collect();
        Ok(events)
    }

    fn load(&self, run_id: &str, seq: u64) -> Result<Option<JournalEvent>, AncoraError> {
        Ok(self.read(run_id)?.into_iter().nth(seq as usize))
    }
}

/// Build a fixture from a slice of (key, kind, input, result) tuples.
pub fn build_fixture(entries: &[(&str, &str, &str, &str)]) -> Fixture {
    let mut f = Fixture::new();
    for (key, kind, input, result) in entries {
        f.add(FixtureEntry {
            activity_key: key.to_string(),
            activity_kind: kind.to_string(),
            input_json: input.to_string(),
            result_json: result.to_string(),
        });
    }
    f
}

/// Replay multiple activities from a fixture in the given key order.
pub fn replay_fixture_sequence(
    fixture: &Fixture,
    keys: &[&str],
) -> Result<Vec<String>, AncoraError> {
    keys.iter().map(|k| replay_fixture(fixture, k)).collect()
}

/// Replay a fixture: returns the recorded result for a given activity key.
/// Returns `Ok(result_json)` for known keys and `Err` for unknown keys.
pub fn replay_fixture(fixture: &Fixture, activity_key: &str) -> Result<String, AncoraError> {
    fixture
        .get_result(activity_key)
        .map(|s| s.to_string())
        .ok_or_else(|| {
            AncoraError::Nondeterminism {
                seq: 0,
                expected: activity_key.to_string(),
                got: "<not in fixture>".to_string(),
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_entry(key: &str, result: &str) -> FixtureEntry {
        FixtureEntry {
            activity_key: key.into(),
            activity_kind: "model_call".into(),
            input_json: "{}".into(),
            result_json: result.into(),
        }
    }

    #[test]
    fn replay_fixture_sequence_replays_in_order() {
        let f = build_fixture(&[
            ("a", "m", "{}", r#""r-a""#),
            ("b", "m", "{}", r#""r-b""#),
        ]);
        let results = replay_fixture_sequence(&f, &["a", "b"]).unwrap();
        assert_eq!(results, vec![r#""r-a""#, r#""r-b""#]);
    }

    #[test]
    fn replay_fixture_sequence_fails_on_missing_key() {
        let f = build_fixture(&[("a", "m", "{}", r#""r-a""#)]);
        assert!(replay_fixture_sequence(&f, &["a", "missing"]).is_err());
    }

    #[test]
    fn fixture_merge_adds_new_entries() {
        let mut a = build_fixture(&[("k1", "m", "{}", r#""r1""#)]);
        let b = build_fixture(&[("k2", "m", "{}", r#""r2""#)]);
        a.merge(b);
        assert_eq!(a.len(), 2);
        assert_eq!(a.get_result("k2"), Some(r#""r2""#));
    }

    #[test]
    fn fixture_merge_overwrites_existing_key() {
        let mut a = build_fixture(&[("k1", "m", "{}", r#""old""#)]);
        let b = build_fixture(&[("k1", "m", "{}", r#""new""#)]);
        a.merge(b);
        assert_eq!(a.len(), 1);
        assert_eq!(a.get_result("k1"), Some(r#""new""#));
    }

    #[test]
    fn fixture_stores_and_retrieves_entries() {
        let mut f = Fixture::new();
        f.add(make_entry("step-1", r#""output1""#));
        assert_eq!(f.get_result("step-1"), Some(r#""output1""#));
        assert_eq!(f.len(), 1);
    }

    #[test]
    fn fixture_returns_none_for_missing_key() {
        let f = Fixture::new();
        assert_eq!(f.get_result("missing"), None);
    }

    #[test]
    fn replay_fixture_returns_recorded_result() {
        let mut f = Fixture::new();
        f.add(make_entry("k1", r#""answer""#));
        let result = replay_fixture(&f, "k1").unwrap();
        assert_eq!(result, r#""answer""#);
    }

    #[test]
    fn replay_fixture_errors_for_unknown_key() {
        let f = Fixture::new();
        assert!(replay_fixture(&f, "not_there").is_err());
    }

    #[test]
    fn fixture_recorder_captures_entries() {
        let recorder = FixtureRecorder::new();
        recorder.record("k1", "model_call", "{}", r#""r1""#);
        recorder.record("k2", "tool_call", "{}", r#""r2""#);
        let f = recorder.into_fixture();
        assert_eq!(f.len(), 2);
        assert_eq!(f.get_result("k1"), Some(r#""r1""#));
    }

    #[test]
    fn fixture_recorder_snapshot_does_not_consume() {
        let recorder = FixtureRecorder::new();
        recorder.record("k1", "model_call", "{}", r#""r1""#);
        let snap = recorder.snapshot();
        assert_eq!(snap.len(), 1);
        recorder.record("k2", "model_call", "{}", r#""r2""#);
        assert_eq!(recorder.snapshot().len(), 2);
    }

    #[test]
    fn build_fixture_constructs_from_tuples() {
        let f = build_fixture(&[("a", "model_call", "{}", r#""x""#)]);
        assert_eq!(f.get_result("a"), Some(r#""x""#));
    }

    #[test]
    fn fixture_journal_store_read_returns_activity_events() {
        let f = build_fixture(&[
            ("step-1", "model_call", "{}", r#""r1""#),
            ("step-2", "tool_call", "{}", r#""r2""#),
        ]);
        let store = FixtureJournalStore::new(f);
        let events = store.read("run-x").unwrap();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].seq, 0);
        assert_eq!(events[1].seq, 1);
    }

    #[test]
    fn fixture_journal_store_load_returns_correct_event() {
        let f = build_fixture(&[("a", "model_call", "{}", r#""r1""#)]);
        let store = FixtureJournalStore::new(f);
        let ev = store.load("run-y", 0).unwrap();
        assert!(ev.is_some());
    }

    #[test]
    fn fixture_journal_store_replayed_flag_is_true() {
        use ancora_proto::ancora::journal_event::Event;
        let f = build_fixture(&[("k", "m", "{}", r#""v""#)]);
        let store = FixtureJournalStore::new(f);
        let events = store.read("r").unwrap();
        match &events[0].event {
            Some(Event::ActivityRecorded(a)) => assert!(a.replayed),
            _ => panic!("expected ActivityRecorded"),
        }
    }

    #[test]
    fn fixture_journal_store_empty_fixture_returns_empty_events() {
        let store = FixtureJournalStore::new(Fixture::new());
        let events = store.read("r").unwrap();
        assert!(events.is_empty());
    }

    #[test]
    fn fixture_journal_store_append_is_error() {
        use ancora_proto::ancora::{journal_event::Event, JournalEvent, RunStartedEvent};
        let store = FixtureJournalStore::new(Fixture::new());
        let ev = JournalEvent {
            event_id: "e".into(),
            run_id: "r".into(),
            seq: 0,
            recorded_at_ns: 0,
            event: Some(Event::RunStarted(RunStartedEvent {
                run_id: "r".into(),
                spec_bytes: vec![],
                spec_type: "AgentSpec".into(),
            })),
        };
        assert!(store.append("r", ev).is_err());
    }

    #[test]
    fn detect_divergence_passes_when_fixture_matches_observed() {
        use crate::replay::detect_divergence;
        let keys = vec!["step-a".to_string(), "step-b".to_string()];
        detect_divergence(&keys, &keys).unwrap();
    }

    #[test]
    fn detect_divergence_catches_fixture_key_mismatch() {
        use crate::replay::detect_divergence;
        let expected = vec!["step-a".to_string()];
        let observed = vec!["step-b".to_string()];
        assert!(detect_divergence(&expected, &observed).is_err());
    }

    #[test]
    fn fixture_journal_store_integrates_with_replay_events() {
        use crate::journal::JournalStore;
        use crate::replay::replay_events;
        let f = build_fixture(&[
            ("step-a", "model_call", "{}", r#""ra""#),
            ("step-b", "model_call", "{}", r#""rb""#),
        ]);
        let store = FixtureJournalStore::new(f);
        let events = store.read("run-z").unwrap();
        let state = replay_events("run-z", &events).unwrap();
        assert_eq!(state.activity_keys, vec!["step-a", "step-b"]);
    }

    #[test]
    fn fixture_is_empty_by_default() {
        let f = Fixture::new();
        assert!(f.is_empty());
        assert_eq!(f.len(), 0);
    }

    #[test]
    fn fixture_entry_serializes_to_json_with_all_fields() {
        let entry = make_entry("k1", r#""val""#);
        let json = serde_json::to_string(&entry).unwrap();
        let decoded: FixtureEntry = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded, entry);
    }

    #[test]
    fn load_fixture_skips_blank_lines() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("fix.jsonl");
        let entry = make_entry("k1", r#""v1""#);
        let line = serde_json::to_string(&entry).unwrap();
        std::fs::write(&path, format!("{}\n\n{}\n", line, line)).unwrap();
        let loaded = load_fixture_from_file(&path).unwrap();
        assert_eq!(loaded.len(), 2);
    }

    #[test]
    fn fixture_file_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("fixture.jsonl");
        let mut f = Fixture::new();
        f.add(make_entry("a", r#""alpha""#));
        f.add(make_entry("b", r#""beta""#));
        record_fixture_to_file(&f, &path).unwrap();
        let loaded = load_fixture_from_file(&path).unwrap();
        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded.get_result("a"), Some(r#""alpha""#));
        assert_eq!(loaded.get_result("b"), Some(r#""beta""#));
    }
}
