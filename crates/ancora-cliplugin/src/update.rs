/// Plugin update mechanism - compares installed plugin versions against
/// available versions and produces update descriptors.
use std::collections::HashMap;

/// Represents a version with major, minor, and patch components.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Version {
    /// Parse a semver string of the form "MAJOR.MINOR.PATCH".
    pub fn parse(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 3 {
            return None;
        }
        let major = parts[0].parse().ok()?;
        let minor = parts[1].parse().ok()?;
        let patch = parts[2].parse().ok()?;
        Some(Self {
            major,
            minor,
            patch,
        })
    }

    /// Format as a "MAJOR.MINOR.PATCH" string.
    pub fn to_string_repr(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.major, self.minor, self.patch).cmp(&(other.major, other.minor, other.patch))
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string_repr())
    }
}

/// A record describing an available update for a plugin.
#[derive(Debug, Clone)]
pub struct UpdateAvailable {
    /// The plugin identifier.
    pub plugin_id: String,
    /// Currently installed version.
    pub installed: Version,
    /// The newer version available.
    pub available: Version,
    /// Optional URL or registry path where the update can be retrieved.
    pub update_url: Option<String>,
    /// Human-readable release notes or changelog excerpt.
    pub notes: Option<String>,
}

/// The status of a plugin with respect to updates.
#[derive(Debug, Clone)]
pub enum UpdateStatus {
    /// An update is available.
    UpdateAvailable(UpdateAvailable),
    /// The plugin is up to date.
    UpToDate { plugin_id: String, version: Version },
    /// The installed version is newer than what the registry reports (pre-release).
    AheadOfRegistry {
        plugin_id: String,
        installed: Version,
        registry: Version,
    },
}

/// A mock registry entry for tests (no network required).
#[derive(Debug, Clone)]
pub struct RegistryEntry {
    pub plugin_id: String,
    pub latest_version: Version,
    pub update_url: Option<String>,
    pub notes: Option<String>,
}

/// An in-memory update registry (production code would fetch from remote).
#[derive(Debug, Default)]
pub struct UpdateRegistry {
    entries: HashMap<String, RegistryEntry>,
}

impl UpdateRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    /// Register an entry in the registry.
    pub fn register(&mut self, entry: RegistryEntry) {
        self.entries.insert(entry.plugin_id.clone(), entry);
    }

    /// Check a single installed plugin against the registry.
    pub fn check(&self, plugin_id: &str, installed_version: &str) -> Option<UpdateStatus> {
        let installed = Version::parse(installed_version)?;
        let entry = self.entries.get(plugin_id)?;
        let latest = &entry.latest_version;

        let status = match installed.cmp(latest) {
            std::cmp::Ordering::Less => UpdateStatus::UpdateAvailable(UpdateAvailable {
                plugin_id: plugin_id.to_string(),
                installed,
                available: latest.clone(),
                update_url: entry.update_url.clone(),
                notes: entry.notes.clone(),
            }),
            std::cmp::Ordering::Equal => UpdateStatus::UpToDate {
                plugin_id: plugin_id.to_string(),
                version: installed,
            },
            std::cmp::Ordering::Greater => UpdateStatus::AheadOfRegistry {
                plugin_id: plugin_id.to_string(),
                installed,
                registry: latest.clone(),
            },
        };
        Some(status)
    }

    /// Check all plugins in a map of `plugin_id -> installed_version`.
    pub fn check_all(&self, installed: &HashMap<String, String>) -> Vec<UpdateStatus> {
        installed
            .iter()
            .filter_map(|(id, ver)| self.check(id, ver))
            .collect()
    }
}
