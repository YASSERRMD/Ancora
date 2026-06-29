//! iOS C-ABI target loader and integration helpers.
//!
//! Provides types and utilities for exposing the on-device runtime
//! to Swift / Objective-C applications via a stable C ABI.

use crate::targets::{IosCabi, TargetTriple};
use serde::{Deserialize, Serialize};

/// State returned when the iOS C-ABI runtime loads successfully.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IosLoadResult {
    /// Target triple resolved at runtime.
    pub triple: String,
    /// Module name of the Swift overlay.
    pub module_name: String,
    /// Minimum iOS version requirement.
    pub min_ios_version: String,
    /// Whether the C-ABI symbols are exported.
    pub cabi_ready: bool,
}

/// Simulate loading the on-device runtime for iOS.
///
/// In production this would be called from the Objective-C
/// `+initialize` or Swift static initialiser.
pub fn ios_load(config: &IosCabi) -> IosLoadResult {
    IosLoadResult {
        triple: TargetTriple::IosArm64.triple().to_string(),
        module_name: config.module_name.clone(),
        min_ios_version: config.min_ios_version.clone(),
        cabi_ready: !config.header_path.is_empty(),
    }
}

/// Parse an iOS version string and return `(major, minor)`.
pub fn parse_ios_version(version: &str) -> Option<(u32, u32)> {
    let mut parts = version.splitn(2, '.');
    let major: u32 = parts.next()?.parse().ok()?;
    let minor: u32 = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    Some((major, minor))
}

/// Return whether a version string meets the minimum.
pub fn meets_min_version(version: &str, min: &str) -> bool {
    match (parse_ios_version(version), parse_ios_version(min)) {
        (Some((vmaj, vmin)), Some((mmaj, mmin))) => {
            (vmaj, vmin) >= (mmaj, mmin)
        }
        _ => false,
    }
}

#[cfg(test)]
mod unit {
    use super::*;

    #[test]
    fn ios_load_returns_ready() {
        let cfg = IosCabi::default_config();
        let result = ios_load(&cfg);
        assert!(result.cabi_ready);
        assert_eq!(result.module_name, "AncoraOndev");
    }

    #[test]
    fn parse_ios_version_major_minor() {
        assert_eq!(parse_ios_version("14.0"), Some((14, 0)));
        assert_eq!(parse_ios_version("16.4"), Some((16, 4)));
        assert_eq!(parse_ios_version("17"), Some((17, 0)));
    }

    #[test]
    fn meets_min_version_logic() {
        assert!(meets_min_version("14.0", "14.0"));
        assert!(meets_min_version("16.0", "14.0"));
        assert!(!meets_min_version("13.0", "14.0"));
    }
}
