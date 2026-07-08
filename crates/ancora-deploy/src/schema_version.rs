use crate::error::DeployError;
use crate::worker::Version;

/// Journal schema version. Workers must negotiate before exchanging entries.
#[derive(Clone, Debug, PartialEq)]
pub struct SchemaVersion(pub u32);

impl SchemaVersion {
    /// Returns the highest schema version supported by a journal entry version.
    pub fn from_worker_version(v: &Version) -> Self {
        // Schema version tracks major worker version
        SchemaVersion(v.major)
    }

    pub fn is_compatible(&self, other: &SchemaVersion) -> bool {
        self.0 == other.0
    }
}

pub fn assert_compatible(a: &Version, b: &Version) -> Result<(), DeployError> {
    if a.is_compatible_with(b) {
        Ok(())
    } else {
        Err(DeployError::IncompatibleVersion {
            a: a.to_string(),
            b: b.to_string(),
        })
    }
}
