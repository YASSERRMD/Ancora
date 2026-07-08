/// A semantic version stored as three numeric components.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Version {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Parse a version string of the form "MAJOR.MINOR.PATCH".
    pub fn parse(s: &str) -> Result<Self, VersionParseError> {
        let parts: Vec<&str> = s.splitn(3, '.').collect();
        if parts.len() != 3 {
            return Err(VersionParseError(format!(
                "expected MAJOR.MINOR.PATCH, got '{s}'"
            )));
        }
        let major = parts[0]
            .parse::<u32>()
            .map_err(|_| VersionParseError(format!("invalid major: '{}'", parts[0])))?;
        let minor = parts[1]
            .parse::<u32>()
            .map_err(|_| VersionParseError(format!("invalid minor: '{}'", parts[1])))?;
        let patch = parts[2]
            .parse::<u32>()
            .map_err(|_| VersionParseError(format!("invalid patch: '{}'", parts[2])))?;
        Ok(Self {
            major,
            minor,
            patch,
        })
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Error returned when a version string cannot be parsed.
#[derive(Debug, PartialEq, Eq)]
pub struct VersionParseError(pub String);

impl std::fmt::Display for VersionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "version parse error: {}", self.0)
    }
}

/// An ordered, deduplicated list of versions for a single registry entry.
#[derive(Debug, Default, Clone)]
pub struct VersionList {
    versions: Vec<Version>,
}

impl VersionList {
    /// Add a version, maintaining sorted order and uniqueness.
    pub fn add(&mut self, version: Version) {
        if !self.versions.contains(&version) {
            self.versions.push(version);
            self.versions.sort();
        }
    }

    /// Return the versions in ascending order.
    pub fn list(&self) -> &[Version] {
        &self.versions
    }

    /// Return the latest (highest) version, if any.
    pub fn latest(&self) -> Option<&Version> {
        self.versions.last()
    }

    /// Return the number of versions recorded.
    pub fn len(&self) -> usize {
        self.versions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.versions.is_empty()
    }
}
