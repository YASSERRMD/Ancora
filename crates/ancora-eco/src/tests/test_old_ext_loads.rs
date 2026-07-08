use crate::semver::SemVer;
use crate::version_negotiation::{negotiate, CoreApiVersion, ExtensionManifest, NegotiationResult};

/// Verifies that an old extension (targeting an earlier minor version)
/// still loads correctly when the core is within the same major version
/// and covered by the extension's max_api_version.
#[test]
fn old_extension_on_new_core_still_loads_within_policy() {
    // An extension built against core 1.0-1.4 (the old range).
    let manifest = ExtensionManifest {
        min_api_version: SemVer::new(1, 0, 0),
        max_api_version: SemVer::new(1, 4, 0),
    };
    // Core has since advanced to 1.3.0, still within the extension's max.
    let core = CoreApiVersion {
        version: SemVer::new(1, 3, 0),
    };
    let result = negotiate(&manifest, &core);
    assert_eq!(
        result,
        NegotiationResult::Compatible,
        "old extension should load when core is within its declared max"
    );
}

/// Verifies that an old extension is marked incompatible when the core
/// advances beyond the extension's declared max API version (same major).
#[test]
fn old_extension_incompatible_when_core_beyond_max() {
    let manifest = ExtensionManifest {
        min_api_version: SemVer::new(1, 0, 0),
        max_api_version: SemVer::new(1, 2, 0),
    };
    let core = CoreApiVersion {
        version: SemVer::new(1, 5, 0),
    };
    let result = negotiate(&manifest, &core);
    assert_eq!(
        result,
        NegotiationResult::ExtensionTooOld,
        "old extension should not load when core exceeds its max API version"
    );
}

/// Verifies that a Stable policy does not permit loading across a major
/// version bump even if the extension was formerly compatible.
#[test]
fn major_version_policy_blocks_old_extension() {
    let manifest = ExtensionManifest {
        min_api_version: SemVer::new(1, 0, 0),
        max_api_version: SemVer::new(1, 9, 0),
    };
    let core = CoreApiVersion {
        version: SemVer::new(2, 0, 0),
    };
    let result = negotiate(&manifest, &core);
    assert_ne!(
        result,
        NegotiationResult::Compatible,
        "major version mismatch must block loading"
    );
}
