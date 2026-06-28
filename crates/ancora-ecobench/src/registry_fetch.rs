//! Registry fetch time measurement.
//!
//! Models the time required to query a plugin registry for available packages.
//! Network I/O is replaced with in-memory data structures so that benchmarks
//! are fully deterministic and offline.

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// A version record stored in the registry.
#[derive(Debug, Clone)]
pub struct VersionRecord {
    /// Version string.
    pub version: String,
    /// SHA-256 digest of the package (hex string).
    pub digest: String,
    /// Size in bytes.
    pub size: usize,
}

/// An index entry for a single plugin in the registry.
#[derive(Debug, Clone)]
pub struct RegistryEntry {
    /// Plugin identifier.
    pub id: String,
    /// All published versions, ordered newest-first.
    pub versions: Vec<VersionRecord>,
}

impl RegistryEntry {
    /// Return the latest version, if any.
    pub fn latest(&self) -> Option<&VersionRecord> {
        self.versions.first()
    }
}

/// An in-memory registry holding a set of plugin entries.
#[derive(Debug, Default)]
pub struct InMemoryRegistry {
    entries: HashMap<String, RegistryEntry>,
}

impl InMemoryRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Seed the registry with an entry.
    pub fn seed(&mut self, entry: RegistryEntry) {
        self.entries.insert(entry.id.clone(), entry);
    }

    /// Number of entries in the registry.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns `true` if the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// Result of a registry fetch operation.
#[derive(Debug)]
pub struct FetchResult {
    /// Entries returned by the query.
    pub entries: Vec<RegistryEntry>,
    /// Total elapsed time.
    pub elapsed: Duration,
    /// Number of entries scanned.
    pub scanned: usize,
}

/// Fetch all registry entries matching the given prefix.
///
/// An empty prefix matches everything.
pub fn fetch(registry: &InMemoryRegistry, prefix: &str) -> FetchResult {
    let start = Instant::now();

    let mut entries: Vec<RegistryEntry> = registry
        .entries
        .values()
        .filter(|e| prefix.is_empty() || e.id.starts_with(prefix))
        .cloned()
        .collect();

    let scanned = registry.entries.len();

    // Sort for deterministic ordering.
    entries.sort_by(|a, b| a.id.cmp(&b.id));

    FetchResult {
        entries,
        elapsed: start.elapsed(),
        scanned,
    }
}

/// Regression threshold for a registry fetch in microseconds.
pub const FETCH_TARGET_US: u64 = 3_000;

/// Returns `true` if the fetch completed within the regression threshold.
pub fn within_target(result: &FetchResult) -> bool {
    result.elapsed.as_micros() as u64 <= FETCH_TARGET_US
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_entry(id: &str) -> RegistryEntry {
        RegistryEntry {
            id: id.to_owned(),
            versions: vec![VersionRecord {
                version: "1.0.0".to_owned(),
                digest: "abc123".to_owned(),
                size: 1024,
            }],
        }
    }

    #[test]
    fn fetch_all_entries() {
        let mut reg = InMemoryRegistry::new();
        reg.seed(make_entry("plugin-a"));
        reg.seed(make_entry("plugin-b"));
        let r = fetch(&reg, "");
        assert_eq!(r.entries.len(), 2);
    }

    #[test]
    fn fetch_with_prefix_filters() {
        let mut reg = InMemoryRegistry::new();
        reg.seed(make_entry("ancora-foo"));
        reg.seed(make_entry("third-party-bar"));
        let r = fetch(&reg, "ancora-");
        assert_eq!(r.entries.len(), 1);
        assert_eq!(r.entries[0].id, "ancora-foo");
    }
}
