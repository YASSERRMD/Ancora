use crate::ts_app::{TsApp, TsAppError};

#[test]
fn ts_app_runs_offline() {
    let app = TsApp::new("ts-agent", "0.28.0").unwrap();
    let trace = app.run("hello typescript").unwrap();
    assert_eq!(trace.messages.len(), 2);
    assert_eq!(trace.messages[0].role, "user");
    assert_eq!(trace.messages[1].role, "assistant");
    assert_eq!(trace.stop_reason, "end_turn");
    assert!(trace.trace_id.starts_with("ts-trace-"));
}

#[test]
fn ts_app_rejects_empty_content() {
    let app = TsApp::new("ts-agent", "0.28.0").unwrap();
    let err = app.run("").unwrap_err();
    assert_eq!(err, TsAppError::EmptyContent);
}

#[test]
fn ts_app_rejects_empty_sdk_version() {
    let err = TsApp::new("ts-agent", "").unwrap_err();
    assert_eq!(err, TsAppError::InvalidSdkVersion);
}

#[test]
fn ts_app_includes_version_in_reply() {
    let app = TsApp::new("ts-agent", "1.2.3").unwrap();
    let trace = app.run("ping").unwrap();
    assert!(trace.messages[1].content.contains("1.2.3"));
}
