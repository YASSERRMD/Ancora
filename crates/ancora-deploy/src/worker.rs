use serde::{Deserialize, Serialize};

/// Semantic version tag attached to each worker.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Version {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self { major, minor, patch }
    }

    pub fn is_compatible_with(&self, other: &Version) -> bool {
        // Same major version; minor is forward-compatible
        self.major == other.major
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// A version-tagged worker in the pool.
#[derive(Clone, Debug)]
pub struct VersionedWorker {
    pub id: String,
    pub version: Version,
    pub active_runs: u32,
}

impl VersionedWorker {
    pub fn new(id: impl Into<String>, version: Version) -> Self {
        Self { id: id.into(), version, active_runs: 0 }
    }

    pub fn is_idle(&self) -> bool {
        self.active_runs == 0
    }
}
