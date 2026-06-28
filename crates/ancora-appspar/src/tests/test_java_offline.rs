use crate::java_app::{JavaApp, JavaAppError};

#[test]
fn java_app_runs_offline() {
    let app = JavaApp::new("java-agent", 21).unwrap();
    let trace = app.run("hello java").unwrap();
    assert_eq!(trace.messages.len(), 2);
    assert_eq!(trace.messages[0].role, "user");
    assert_eq!(trace.messages[1].role, "assistant");
    assert_eq!(trace.java_version, 21);
    assert!(trace.trace_id.starts_with("java-trace-"));
}

#[test]
fn java_app_rejects_empty_content() {
    let app = JavaApp::new("java-agent", 17).unwrap();
    let err = app.run("").unwrap_err();
    assert_eq!(err, JavaAppError::EmptyContent);
}

#[test]
fn java_app_rejects_old_version() {
    let err = JavaApp::new("java-agent", 8).unwrap_err();
    assert_eq!(err, JavaAppError::UnsupportedJavaVersion(8));
}

#[test]
fn java_app_accepts_java_11() {
    let app = JavaApp::new("java-agent", 11).unwrap();
    assert_eq!(app.java_version, 11);
}
