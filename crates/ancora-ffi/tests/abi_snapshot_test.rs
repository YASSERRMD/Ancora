use std::path::PathBuf;

fn snapshot_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("include/ancora.h")
}

#[test]
fn snapshot_header_exists() {
    assert!(snapshot_path().exists(), "include/ancora.h not found - run cargo build -p ancora-ffi");
}

#[test]
fn snapshot_header_is_not_empty() {
    let content = std::fs::read_to_string(snapshot_path()).expect("failed to read ancora.h");
    assert!(!content.is_empty(), "ancora.h should not be empty");
}
