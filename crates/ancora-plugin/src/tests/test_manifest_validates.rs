//! Tests: manifest validation.

use crate::manifest::{ManifestBuilder, ManifestError, PluginKind, SemVer};

#[test]
fn valid_manifest_builds_successfully() {
    let m = ManifestBuilder::new()
        .id("test-provider")
        .name("Test Provider")
        .version(SemVer::new(1, 0, 0))
        .sdk_range(SemVer::new(1, 0, 0), SemVer::new(1, 99, 0))
        .kind(PluginKind::Provider)
        .author("YASSERRMD")
        .description("A test provider plugin.")
        .scope("llm:generate")
        .build();
    assert!(m.is_ok(), "expected Ok, got {m:?}");
    let m = m.unwrap();
    assert_eq!(m.id, "test-provider");
    assert_eq!(m.kind, PluginKind::Provider);
    assert_eq!(m.required_scopes, vec!["llm:generate"]);
}

#[test]
fn missing_id_returns_error() {
    let result = ManifestBuilder::new()
        .name("No ID")
        .version(SemVer::new(1, 0, 0))
        .sdk_range(SemVer::new(1, 0, 0), SemVer::new(1, 0, 0))
        .kind(PluginKind::Tool)
        .build();
    assert_eq!(result, Err(ManifestError::MissingField("id".into())));
}

#[test]
fn id_with_spaces_is_invalid() {
    let result = ManifestBuilder::new()
        .id("bad id")
        .name("Bad")
        .version(SemVer::new(1, 0, 0))
        .sdk_range(SemVer::new(1, 0, 0), SemVer::new(2, 0, 0))
        .kind(PluginKind::Tool)
        .build();
    assert!(matches!(result, Err(ManifestError::InvalidId(_))));
}

#[test]
fn inverted_sdk_range_is_rejected() {
    let result = ManifestBuilder::new()
        .id("inverted")
        .name("Inverted SDK range")
        .version(SemVer::new(1, 0, 0))
        .sdk_range(SemVer::new(2, 0, 0), SemVer::new(1, 0, 0))
        .kind(PluginKind::Grader)
        .build();
    assert_eq!(result, Err(ManifestError::SdkRangeInverted));
}

#[test]
fn semver_parse_roundtrip() {
    let v = SemVer::parse("3.14.159").unwrap();
    assert_eq!(v.major, 3);
    assert_eq!(v.minor, 14);
    assert_eq!(v.patch, 159);
    assert_eq!(v.to_string(), "3.14.159");
}

#[test]
fn semver_parse_invalid_returns_error() {
    assert!(SemVer::parse("1.2").is_err());
    assert!(SemVer::parse("1.2.x").is_err());
    assert!(SemVer::parse("").is_err());
}
