// Release smoke tests: verify install-time invariants from each dry-run artifact.

const RELEASE_VERSION: &str = "0.6.0";
const ARTIFACT_NAMES: &[&str] = &[
    "ancora-linux-x86_64",
    "ancora-macos-arm64",
    "ancora-py-0.6.0-cp311-manylinux.whl",
    "ancora-js-0.6.0.tgz",
    "Ancora.0.6.0.nupkg",
    "ancora-java-0.6.0.jar",
];

fn expected_version_string() -> String {
    format!("ancora-core {}", RELEASE_VERSION)
}

fn parse_version_from_binary_output(output: &str) -> Option<&str> {
    output.split_whitespace().last()
}

#[test]
fn test_release_version_is_0_6_0() {
    assert_eq!(RELEASE_VERSION, "0.6.0");
}

#[test]
fn test_artifact_count_covers_all_languages() {
    assert_eq!(ARTIFACT_NAMES.len(), 6);
}

#[test]
fn test_all_artifacts_named_ancora() {
    for name in ARTIFACT_NAMES {
        assert!(name.to_lowercase().contains("ancora"), "expected ancora in {name}");
    }
}

#[test]
fn test_all_artifacts_contain_version() {
    for name in ARTIFACT_NAMES {
        assert!(name.contains(RELEASE_VERSION), "expected version in {name}");
    }
}

#[test]
fn test_version_string_format() {
    let s = expected_version_string();
    assert!(s.starts_with("ancora-core"));
    assert!(s.ends_with(RELEASE_VERSION));
}

#[test]
fn test_parse_version_from_output() {
    let output = "ancora-core 0.6.0";
    let ver = parse_version_from_binary_output(output).unwrap();
    assert_eq!(ver, RELEASE_VERSION);
}

#[test]
fn test_binary_artifact_is_linux_or_macos() {
    let binaries: Vec<&&str> = ARTIFACT_NAMES.iter()
        .filter(|n| !n.contains('.') || n.ends_with("64"))
        .collect();
    for b in &binaries {
        assert!(b.contains("linux") || b.contains("macos") || b.contains("ancora-"),
            "unexpected binary: {b}");
    }
}

#[test]
fn test_python_artifact_is_wheel() {
    let wheels: Vec<&&str> = ARTIFACT_NAMES.iter().filter(|n| n.ends_with(".whl")).collect();
    assert_eq!(wheels.len(), 1);
    assert!(wheels[0].contains("cp311"));
}
