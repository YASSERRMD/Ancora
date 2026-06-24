use std::path::PathBuf;

fn snapshot_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("include/ancora.h")
}

#[test]
fn snapshot_header_exists() {
    assert!(snapshot_path().exists(), "include/ancora.h not found - run cargo build -p ancora-ffi");
}
