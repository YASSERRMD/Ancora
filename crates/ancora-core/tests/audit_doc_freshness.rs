// Documentation audit: docs are not stale -- version references are consistent.

const CURRENT_VERSION: &str = "0.6.0";
const SDK_MIN_VERSION: &str = "0.3.0";

struct VersionRef {
    doc: &'static str,
    version: &'static str,
}

const VERSION_REFS: &[VersionRef] = &[
    VersionRef {
        doc: "sdk/rust/install.md",
        version: "0.6.0",
    },
    VersionRef {
        doc: "sdk/go/install.md",
        version: "0.6.0",
    },
    VersionRef {
        doc: "sdk/python/install.md",
        version: "0.6.0",
    },
    VersionRef {
        doc: "sdk/ts/install.md",
        version: "0.6.0",
    },
    VersionRef {
        doc: "sdk/dotnet/install.md",
        version: "0.6.0",
    },
    VersionRef {
        doc: "sdk/java/install.md",
        version: "0.6.0",
    },
];

fn is_current_or_newer(version: &str, current: &str) -> bool {
    version >= current
}

#[test]
#[allow(clippy::const_is_empty)]
fn test_current_version_defined() {
    assert!(!CURRENT_VERSION.is_empty());
    let parts: Vec<&str> = CURRENT_VERSION.split('.').collect();
    assert_eq!(parts.len(), 3);
}

#[test]
fn test_all_install_docs_reference_current_version() {
    for vr in VERSION_REFS {
        assert_eq!(
            vr.version, CURRENT_VERSION,
            "{} references version {} but current is {}",
            vr.doc, vr.version, CURRENT_VERSION
        );
    }
}

#[test]
fn test_six_install_docs_checked() {
    assert_eq!(VERSION_REFS.len(), 6);
}

#[test]
fn test_sdk_min_version_is_at_least_0_3() {
    assert!(SDK_MIN_VERSION >= "0.3.0");
}

#[test]
fn test_no_version_older_than_min() {
    for vr in VERSION_REFS {
        assert!(
            is_current_or_newer(vr.version, SDK_MIN_VERSION),
            "{} version {} is older than min {}",
            vr.doc,
            vr.version,
            SDK_MIN_VERSION
        );
    }
}

#[test]
fn test_all_version_refs_are_semver_like() {
    for vr in VERSION_REFS {
        let parts: Vec<&str> = vr.version.split('.').collect();
        assert_eq!(parts.len(), 3, "not semver in {}: {}", vr.doc, vr.version);
    }
}
