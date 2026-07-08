/// A simple semantic version representation (major.minor.patch).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemVer {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl SemVer {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        SemVer {
            major,
            minor,
            patch,
        }
    }

    /// Returns true if this version is compatible with `other` (same major, this >= other).
    pub fn is_compatible_with(&self, other: &SemVer) -> bool {
        self.major == other.major && self >= other
    }

    /// Returns true if bumping from `old` to `new` constitutes a breaking change.
    pub fn is_breaking_bump(old: &SemVer, new: &SemVer) -> bool {
        new.major > old.major
    }
}

impl std::fmt::Display for SemVer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Parse a version string of the form "major.minor.patch".
pub fn parse_semver(s: &str) -> Result<SemVer, String> {
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() != 3 {
        return Err(format!("invalid semver: {}", s));
    }
    let major = parts[0].parse::<u32>().map_err(|e| e.to_string())?;
    let minor = parts[1].parse::<u32>().map_err(|e| e.to_string())?;
    let patch = parts[2].parse::<u32>().map_err(|e| e.to_string())?;
    Ok(SemVer::new(major, minor, patch))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_and_display() {
        let v = parse_semver("1.2.3").unwrap();
        assert_eq!(v.to_string(), "1.2.3");
    }

    #[test]
    fn major_bump_is_breaking() {
        let old = SemVer::new(1, 5, 0);
        let new = SemVer::new(2, 0, 0);
        assert!(SemVer::is_breaking_bump(&old, &new));
    }

    #[test]
    fn minor_bump_is_not_breaking() {
        let old = SemVer::new(1, 0, 0);
        let new = SemVer::new(1, 1, 0);
        assert!(!SemVer::is_breaking_bump(&old, &new));
    }
}
