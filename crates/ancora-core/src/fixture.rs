use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::AncoraError;

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
