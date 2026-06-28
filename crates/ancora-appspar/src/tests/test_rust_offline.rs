use crate::rust_app::{RustApp, RustAppError};

#[test]
fn rust_app_runs_offline() {
    let app = RustApp::new("rust-agent", 2021).unwrap();
    let trace = app.run("hello rust").unwrap();
    assert_eq!(trace.messages.len(), 2);
    assert_eq!(trace.messages[0].role, "user");
    assert_eq!(trace.messages[1].role, "assistant");
    assert_eq!(trace.edition, 2021);
    assert!(trace.trace_id.starts_with("rust-trace-"));
}

#[test]
fn rust_app_rejects_empty_content() {
    let app = RustApp::new("rust-agent", 2021).unwrap();
    let err = app.run("").unwrap_err();
    assert_eq!(err, RustAppError::EmptyContent);
}

#[test]
fn rust_app_rejects_bad_edition() {
    let err = RustApp::new("rust-agent", 2015).unwrap_err();
    assert_eq!(err, RustAppError::UnsupportedEdition(2015));
}

#[test]
fn rust_app_accepts_2018_edition() {
    let app = RustApp::new("rust-agent", 2018).unwrap();
    assert_eq!(app.edition, 2018);
}
