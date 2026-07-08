/// Plugin version compatibility check.
use crate::manifest::{ManifestError, PluginManifest, SemVer};

/// Error returned when a compatibility check fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompatError {
    IncompatibleSdk {
        sdk_version: SemVer,
        min_required: SemVer,
        max_allowed: SemVer,
    },
    ManifestError(ManifestError),
}

impl From<ManifestError> for CompatError {
    fn from(e: ManifestError) -> Self {
        CompatError::ManifestError(e)
    }
}

impl std::fmt::Display for CompatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompatError::IncompatibleSdk {
                sdk_version,
                min_required,
                max_allowed,
            } => write!(
                f,
                "SDK {sdk_version} is outside plugin range [{min_required}, {max_allowed}]"
            ),
            CompatError::ManifestError(e) => write!(f, "manifest error: {e}"),
        }
    }
}

impl std::error::Error for CompatError {}

/// The current SDK version provided by this crate.
pub const CURRENT_SDK_VERSION: SemVer = SemVer {
    major: 1,
    minor: 0,
    patch: 0,
};

/// Check that a plugin manifest is compatible with the given SDK version.
///
/// A plugin is compatible when `min_sdk <= sdk_version <= max_sdk` (inclusive both ends,
/// using major-only comparison for the upper bound to allow minor-version additions).
pub fn check_compatibility(
    manifest: &PluginManifest,
    sdk_version: &SemVer,
) -> Result<(), CompatError> {
    if sdk_version < &manifest.min_sdk || sdk_version > &manifest.max_sdk {
        return Err(CompatError::IncompatibleSdk {
            sdk_version: sdk_version.clone(),
            min_required: manifest.min_sdk.clone(),
            max_allowed: manifest.max_sdk.clone(),
        });
    }
    Ok(())
}

/// Check a manifest against the current SDK version embedded in this crate.
pub fn check_current(manifest: &PluginManifest) -> Result<(), CompatError> {
    check_compatibility(manifest, &CURRENT_SDK_VERSION)
}
