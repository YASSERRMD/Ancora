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

#[test]
fn snapshot_header_contains_include_guard() {
    let content = std::fs::read_to_string(snapshot_path()).expect("failed to read ancora.h");
    assert!(content.contains("#ifndef ANCORA_H"), "missing include guard");
    assert!(content.contains("#define ANCORA_H"), "missing include guard define");
}

#[test]
fn snapshot_header_declares_ancora_create_runtime() {
    let content = std::fs::read_to_string(snapshot_path()).expect("failed to read ancora.h");
    assert!(content.contains("ancora_create_runtime"), "ancora_create_runtime missing from header");
}

#[test]
fn snapshot_header_declares_ancora_free_runtime() {
    let content = std::fs::read_to_string(snapshot_path()).expect("failed to read ancora.h");
    assert!(content.contains("ancora_free_runtime"), "ancora_free_runtime missing from header");
}

#[test]
fn snapshot_header_declares_tool_ops() {
    let content = std::fs::read_to_string(snapshot_path()).expect("failed to read ancora.h");
    assert!(content.contains("ancora_tool_register"), "ancora_tool_register missing");
    assert!(content.contains("ancora_tool_invoke"), "ancora_tool_invoke missing");
    assert!(content.contains("ancora_tool_unregister"), "ancora_tool_unregister missing");
}

#[test]
fn snapshot_header_declares_ancorerrorcode_enum() {
    let content = std::fs::read_to_string(snapshot_path()).expect("failed to read ancora.h");
    assert!(content.contains("AncorErrorCode"), "AncorErrorCode missing from header");
}

#[test]
fn snapshot_header_declares_ancorbuffer_struct() {
    let content = std::fs::read_to_string(snapshot_path()).expect("failed to read ancora.h");
    assert!(content.contains("AncorBuffer"), "AncorBuffer missing from header");
}

#[test]
fn snapshot_header_declares_run_ops() {
    let content = std::fs::read_to_string(snapshot_path()).expect("failed to read ancora.h");
    assert!(content.contains("ancora_run_start"), "ancora_run_start missing");
    assert!(content.contains("ancora_run_poll"), "ancora_run_poll missing");
    assert!(content.contains("ancora_run_resume"), "ancora_run_resume missing");
}

#[test]
fn snapshot_header_declares_ancora_version_function() {
    let content = std::fs::read_to_string(snapshot_path()).expect("failed to read ancora.h");
    assert!(content.contains("ancora_version"), "ancora_version missing from header");
}

#[test]
fn snapshot_header_declares_runtime_new() {
    let content = std::fs::read_to_string(snapshot_path()).expect("failed to read ancora.h");
    assert!(content.contains("ancora_runtime_new"), "ancora_runtime_new missing from header");
}

#[test]
fn snapshot_header_run_start_signature_has_out_param() {
    let content = std::fs::read_to_string(snapshot_path()).expect("failed to read ancora.h");
    let idx = content.find("ancora_run_start").unwrap();
    let end = (idx + 400).min(content.len());
    let sig = &content[idx..end];
    assert!(sig.contains("AncorBuffer"), "ancora_run_start should have AncorBuffer out param");
}
