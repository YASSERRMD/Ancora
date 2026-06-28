use crate::semver::SemVer;

/// Represents what versions an extension declares it supports.
pub struct ExtensionManifest {
    pub min_api_version: SemVer,
    pub max_api_version: SemVer,
}

/// Represents the API version offered by the host core.
pub struct CoreApiVersion {
    pub version: SemVer,
}

/// Outcome of a version negotiation attempt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NegotiationResult {
    /// Negotiation succeeded; extensions and core are compatible.
    Compatible,
    /// The extension requires a newer API than the core provides.
    CoreTooOld,
    /// The extension is too old for the core (below min required).
    ExtensionTooOld,
    /// Major version mismatch; incompatible.
    MajorMismatch,
}

/// Negotiate compatibility between an extension manifest and the core API.
pub fn negotiate(manifest: &ExtensionManifest, core: &CoreApiVersion) -> NegotiationResult {
    if manifest.min_api_version.major != core.version.major {
        return NegotiationResult::MajorMismatch;
    }
    if core.version < manifest.min_api_version {
        return NegotiationResult::CoreTooOld;
    }
    if core.version > manifest.max_api_version {
        // Core is newer than the extension's max - treat as extension too old
        // only if the max_api_version major matches.
        if manifest.max_api_version.major == core.version.major {
            return NegotiationResult::ExtensionTooOld;
        }
        return NegotiationResult::MajorMismatch;
    }
    NegotiationResult::Compatible
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compatible_when_core_in_range() {
        let manifest = ExtensionManifest {
            min_api_version: SemVer::new(1, 0, 0),
            max_api_version: SemVer::new(1, 5, 0),
        };
        let core = CoreApiVersion { version: SemVer::new(1, 3, 0) };
        assert_eq!(negotiate(&manifest, &core), NegotiationResult::Compatible);
    }

    #[test]
    fn major_mismatch_detected() {
        let manifest = ExtensionManifest {
            min_api_version: SemVer::new(1, 0, 0),
            max_api_version: SemVer::new(1, 9, 0),
        };
        let core = CoreApiVersion { version: SemVer::new(2, 0, 0) };
        assert_eq!(negotiate(&manifest, &core), NegotiationResult::MajorMismatch);
    }
}
