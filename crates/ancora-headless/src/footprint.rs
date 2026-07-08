//! Minimal dependency footprint tracking for headless OS integration.
//!
//! Tracks the binary size, dependency count, and runtime memory
//! to ensure Ancora stays within the target for embedded/inference OS use.

/// Target limits for the headless build.
pub struct FootprintTarget {
    /// Maximum binary size in bytes.
    pub max_binary_bytes: u64,
    /// Maximum number of runtime dependencies.
    pub max_deps: usize,
    /// Maximum resident set size in MB at boot.
    pub max_rss_mb: u64,
    /// Maximum disk footprint in MB (binary + assets).
    pub max_disk_mb: u64,
}

impl Default for FootprintTarget {
    fn default() -> Self {
        FootprintTarget {
            max_binary_bytes: 50 * 1024 * 1024, // 50 MB
            max_deps: 50,
            max_rss_mb: 256,
            max_disk_mb: 512,
        }
    }
}

/// Actual measured footprint of the headless binary.
#[derive(Debug, Clone)]
pub struct FootprintMeasurement {
    pub binary_bytes: u64,
    pub dep_count: usize,
    pub rss_mb: u64,
    pub disk_mb: u64,
    pub label: String,
}

impl FootprintMeasurement {
    pub fn new(
        label: impl Into<String>,
        binary_bytes: u64,
        dep_count: usize,
        rss_mb: u64,
        disk_mb: u64,
    ) -> Self {
        FootprintMeasurement {
            binary_bytes,
            dep_count,
            rss_mb,
            disk_mb,
            label: label.into(),
        }
    }

    /// Returns a human-readable binary size string.
    pub fn binary_size_human(&self) -> String {
        let kb = self.binary_bytes / 1024;
        let mb = kb / 1024;
        if mb > 0 {
            format!("{} MB", mb)
        } else {
            format!("{} KB", kb)
        }
    }
}

/// Result of comparing a measurement against a target.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FootprintStatus {
    WithinTarget,
    Exceeded(Vec<String>),
}

/// Checks whether a measurement is within the given target.
pub fn check_footprint(m: &FootprintMeasurement, t: &FootprintTarget) -> FootprintStatus {
    let mut violations = Vec::new();
    if m.binary_bytes > t.max_binary_bytes {
        violations.push(format!(
            "binary size {} > {} bytes",
            m.binary_bytes, t.max_binary_bytes
        ));
    }
    if m.dep_count > t.max_deps {
        violations.push(format!("dep count {} > {}", m.dep_count, t.max_deps));
    }
    if m.rss_mb > t.max_rss_mb {
        violations.push(format!("RSS {} MB > {} MB", m.rss_mb, t.max_rss_mb));
    }
    if m.disk_mb > t.max_disk_mb {
        violations.push(format!("disk {} MB > {} MB", m.disk_mb, t.max_disk_mb));
    }
    if violations.is_empty() {
        FootprintStatus::WithinTarget
    } else {
        FootprintStatus::Exceeded(violations)
    }
}

/// Dependency record for the minimal footprint manifest.
#[derive(Debug, Clone)]
pub struct DepRecord {
    pub name: String,
    pub version: String,
    pub optional: bool,
    pub feature_gated: bool,
}

impl DepRecord {
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        DepRecord {
            name: name.into(),
            version: version.into(),
            optional: false,
            feature_gated: false,
        }
    }

    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }

    pub fn feature_gated(mut self) -> Self {
        self.feature_gated = true;
        self
    }
}

/// A manifest of all compile-time dependencies in the headless build.
pub struct FootprintManifest {
    pub deps: Vec<DepRecord>,
}

impl FootprintManifest {
    pub fn new() -> Self {
        FootprintManifest { deps: Vec::new() }
    }

    pub fn add(&mut self, dep: DepRecord) {
        self.deps.push(dep);
    }

    pub fn count(&self) -> usize {
        self.deps.len()
    }

    pub fn mandatory_count(&self) -> usize {
        self.deps
            .iter()
            .filter(|d| !d.optional && !d.feature_gated)
            .count()
    }
}

impl Default for FootprintManifest {
    fn default() -> Self {
        Self::new()
    }
}
