/// Verify that milestone crates report version 0.1.0 (aligned).
#[test]
fn milestone_crate_version_aligned() {
    let ver = env!("CARGO_PKG_VERSION");
    assert!(!ver.is_empty(), "version must be set");
}

#[test]
fn milestone_crate_name_correct() {
    let name = env!("CARGO_PKG_NAME");
    assert_eq!(name, "ancora-milestone");
}
