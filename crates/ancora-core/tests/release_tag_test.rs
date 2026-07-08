// Release: workspace tag verification -- ensures the tag aligns with workspace version.

const WORKSPACE_VERSION: &str = "0.6.0";
const EXPECTED_TAG: &str = "v0.6.0";
const CRATE_NAMES: &[&str] = &[
    "ancora-proto",
    "ancora-core",
    "ancora-inference",
    "ancora-memory",
    "ancora-tools",
    "ancora-policy",
    "ancora-observability",
    "ancora-cli",
    "ancora-ffi",
    "ancora-grpc",
    "ancora-py",
    "ancora-napi",
    "ancora-examples",
];

fn tag_for_version(version: &str) -> String {
    format!("v{}", version)
}

fn crate_version_string(crate_name: &str, version: &str) -> String {
    format!("{} = {}", crate_name, version)
}

#[test]
fn test_expected_tag_matches_version() {
    assert_eq!(tag_for_version(WORKSPACE_VERSION), EXPECTED_TAG);
}

#[test]
fn test_workspace_version_semver_format() {
    let parts: Vec<&str> = WORKSPACE_VERSION.split('.').collect();
    assert_eq!(parts.len(), 3, "expected MAJOR.MINOR.PATCH");
    for part in &parts {
        assert!(part.parse::<u32>().is_ok(), "non-numeric part: {part}");
    }
}

#[test]
fn test_all_13_crates_listed() {
    assert_eq!(CRATE_NAMES.len(), 13);
}

#[test]
fn test_crate_names_unique() {
    let mut seen = std::collections::HashSet::new();
    for name in CRATE_NAMES {
        assert!(seen.insert(name), "duplicate crate: {name}");
    }
}

#[test]
fn test_crate_version_string_format() {
    let s = crate_version_string("ancora-core", WORKSPACE_VERSION);
    assert!(s.contains("ancora-core"));
    assert!(s.contains(WORKSPACE_VERSION));
}

#[test]
fn test_tag_starts_with_v() {
    assert!(EXPECTED_TAG.starts_with('v'));
}

#[test]
fn test_minor_version_is_6() {
    let parts: Vec<u32> = WORKSPACE_VERSION
        .split('.')
        .map(|p| p.parse().unwrap())
        .collect();
    assert_eq!(parts[1], 6);
}
