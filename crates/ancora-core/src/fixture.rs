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

/// Replay a fixture: returns a generator that replays recorded results.
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
