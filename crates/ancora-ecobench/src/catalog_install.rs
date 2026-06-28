//! Catalog install time measurement.
//!
//! Models the time required to resolve, download (simulated), and register a
//! plugin package from a catalog. All network I/O is bypassed; the module
//! uses in-memory byte arrays as stand-ins for package tarballs.

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// A package entry present in the catalog.
#[derive(Debug, Clone)]
pub struct CatalogEntry {
    /// Unique package identifier.
    pub id: String,
    /// Version string in semver format.
    pub version: String,
    /// Compressed size in bytes (used for simulated download).
    pub compressed_size: usize,
}

impl CatalogEntry {
    /// Construct a new catalog entry.
    pub fn new(id: &str, version: &str, compressed_size: usize) -> Self {
        Self {
            id: id.to_owned(),
            version: version.to_owned(),
            compressed_size,
        }
    }
}

/// Simulated local package registry populated by installs.
#[derive(Debug, Default)]
pub struct LocalRegistry {
    entries: HashMap<String, CatalogEntry>,
}

impl LocalRegistry {
    /// Create an empty local registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if a package is already installed.
    pub fn is_installed(&self, id: &str) -> bool {
        self.entries.contains_key(id)
    }

    /// Return a reference to an installed entry, if present.
    pub fn get(&self, id: &str) -> Option<&CatalogEntry> {
        self.entries.get(id)
    }

    /// Number of installed packages.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns `true` if no packages are installed.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// Result of a catalog install operation.
#[derive(Debug)]
pub struct InstallResult {
    /// The entry that was installed.
    pub entry: CatalogEntry,
    /// Whether this was a fresh install (as opposed to a cache hit).
    pub was_download: bool,
    /// Total elapsed time.
    pub elapsed: Duration,
    /// Time spent in the simulated download phase.
    pub download_time: Duration,
    /// Time spent registering the package locally.
    pub register_time: Duration,
}

/// Install a catalog entry into the local registry.
///
/// If the package is already present this function returns quickly (cache hit).
pub fn install(registry: &mut LocalRegistry, entry: CatalogEntry) -> InstallResult {
    let overall = Instant::now();

    // Cache hit: no download needed.
    if registry.is_installed(&entry.id) {
        let cached = registry.get(&entry.id).unwrap().clone();
        return InstallResult {
            entry: cached,
            was_download: false,
            elapsed: overall.elapsed(),
            download_time: Duration::ZERO,
            register_time: Duration::ZERO,
        };
    }

    // Simulate download by iterating over a synthetic byte range.
    let t = Instant::now();
    let _bytes: u64 = (0..entry.compressed_size as u64).fold(0, |a, b| a ^ b);
    let download_time = t.elapsed();

    // Register locally.
    let t = Instant::now();
    registry.entries.insert(entry.id.clone(), entry.clone());
    let register_time = t.elapsed();

    InstallResult {
        entry,
        was_download: true,
        elapsed: overall.elapsed(),
        download_time,
        register_time,
    }
}

/// Regression threshold for a single catalog install in microseconds.
pub const INSTALL_TARGET_US: u64 = 10_000;

/// Returns `true` if the install completed within the regression threshold.
pub fn within_target(result: &InstallResult) -> bool {
    result.elapsed.as_micros() as u64 <= INSTALL_TARGET_US
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn install_registers_entry() {
        let mut reg = LocalRegistry::new();
        let entry = CatalogEntry::new("pkg-a", "1.0.0", 512);
        install(&mut reg, entry);
        assert!(reg.is_installed("pkg-a"));
    }

    #[test]
    fn second_install_is_cache_hit() {
        let mut reg = LocalRegistry::new();
        let e1 = CatalogEntry::new("pkg-b", "2.0.0", 256);
        let e2 = CatalogEntry::new("pkg-b", "2.0.0", 256);
        install(&mut reg, e1);
        let r = install(&mut reg, e2);
        assert!(!r.was_download);
    }
}
