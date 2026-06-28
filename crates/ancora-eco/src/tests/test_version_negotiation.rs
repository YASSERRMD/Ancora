use crate::semver::SemVer;
use crate::version_negotiation::{negotiate, CoreApiVersion, ExtensionManifest, NegotiationResult};

#[test]
fn version_negotiation_selects_compatible_api() {
    let manifest = ExtensionManifest {
        min_api_version: SemVer::new(1, 2, 0),
        max_api_version: SemVer::new(1, 8, 0),
    };
    let core = CoreApiVersion {
        version: SemVer::new(1, 5, 0),
    };
    assert_eq!(negotiate(&manifest, &core), NegotiationResult::Compatible);
}

#[test]
fn negotiation_fails_when_core_too_old() {
    let manifest = ExtensionManifest {
        min_api_version: SemVer::new(1, 6, 0),
        max_api_version: SemVer::new(1, 9, 0),
    };
    let core = CoreApiVersion {
        version: SemVer::new(1, 4, 0),
    };
    assert_eq!(negotiate(&manifest, &core), NegotiationResult::CoreTooOld);
}

#[test]
fn negotiation_fails_on_major_mismatch() {
    let manifest = ExtensionManifest {
        min_api_version: SemVer::new(1, 0, 0),
        max_api_version: SemVer::new(1, 9, 0),
    };
    let core = CoreApiVersion {
        version: SemVer::new(2, 0, 0),
    };
    assert_eq!(negotiate(&manifest, &core), NegotiationResult::MajorMismatch);
}
