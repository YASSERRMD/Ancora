/// Version and changelog metadata for marketplace extensions.
///
/// Tracks the version history of an extension including changelog entries,
/// deprecation notices, and yanked versions.

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemVer {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

impl SemVer {
    pub fn parse(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 3 {
            return None;
        }
        let major = parts[0].parse().ok()?;
        let minor = parts[1].parse().ok()?;
        let patch = parts[2].parse().ok()?;
        Some(SemVer {
            major,
            minor,
            patch,
        })
    }
}

impl std::fmt::Display for SemVer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChangelogEntry {
    pub version: SemVer,
    /// ISO-8601 release date, e.g. "2026-06-01".
    pub released_on: String,
    /// Human-readable summary of changes in this version.
    pub summary: String,
    /// Whether this version has been yanked (removed from the registry).
    pub yanked: bool,
}

#[derive(Debug, Clone, Default)]
pub struct VersionHistory {
    entries: Vec<ChangelogEntry>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VersioningError {
    ParseError(String),
    DuplicateVersion(SemVer),
    VersionNotFound(SemVer),
}

impl std::fmt::Display for VersioningError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VersioningError::ParseError(s) => write!(f, "cannot parse version: '{}'", s),
            VersioningError::DuplicateVersion(v) => write!(f, "version {} already exists", v),
            VersioningError::VersionNotFound(v) => write!(f, "version {} not found", v),
        }
    }
}

impl VersionHistory {
    pub fn new() -> Self {
        VersionHistory {
            entries: Vec::new(),
        }
    }

    /// Add a changelog entry.
    pub fn add(&mut self, entry: ChangelogEntry) -> Result<(), VersioningError> {
        if self.entries.iter().any(|e| e.version == entry.version) {
            return Err(VersioningError::DuplicateVersion(entry.version));
        }
        self.entries.push(entry);
        Ok(())
    }

    /// Mark a version as yanked.
    pub fn yank(&mut self, version: &SemVer) -> Result<(), VersioningError> {
        let entry = self
            .entries
            .iter_mut()
            .find(|e| &e.version == version)
            .ok_or_else(|| VersioningError::VersionNotFound(version.clone()))?;
        entry.yanked = true;
        Ok(())
    }

    /// Return the latest non-yanked version, if any.
    pub fn latest(&self) -> Option<&ChangelogEntry> {
        self.entries
            .iter()
            .filter(|e| !e.yanked)
            .max_by(|a, b| a.version.cmp(&b.version))
    }

    pub fn all(&self) -> &[ChangelogEntry] {
        &self.entries
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semver_parses() {
        let v = SemVer::parse("1.2.3").unwrap();
        assert_eq!(
            v,
            SemVer {
                major: 1,
                minor: 2,
                patch: 3
            }
        );
    }

    #[test]
    fn latest_skips_yanked() {
        let mut h = VersionHistory::new();
        h.add(ChangelogEntry {
            version: SemVer::parse("1.0.0").unwrap(),
            released_on: "2026-01-01".to_string(),
            summary: "Initial release".to_string(),
            yanked: false,
        })
        .unwrap();
        h.add(ChangelogEntry {
            version: SemVer::parse("1.1.0").unwrap(),
            released_on: "2026-02-01".to_string(),
            summary: "Bug fixes".to_string(),
            yanked: false,
        })
        .unwrap();
        h.yank(&SemVer::parse("1.1.0").unwrap()).unwrap();
        let latest = h.latest().unwrap();
        assert_eq!(latest.version, SemVer::parse("1.0.0").unwrap());
    }
}
