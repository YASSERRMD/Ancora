use crate::go_app::{GoApp, GoAppError};

#[test]
fn go_app_runs_offline() {
    let app = GoApp::new("go-agent");
    let trace = app.run("hello world").unwrap();
    assert_eq!(trace.messages.len(), 2);
    assert_eq!(trace.messages[0].role, "user");
    assert_eq!(trace.messages[1].role, "assistant");
    assert!(trace.trace_id.starts_with("go-trace-"));
}

#[test]
fn go_app_rejects_empty_content() {
    let app = GoApp::new("go-agent");
    let err = app.run("").unwrap_err();
    assert_eq!(err, GoAppError::EmptyContent);
}

#[test]
fn go_message_rejects_unknown_role() {
    let app = GoApp::new("go-agent");
    let err = app.create_message("bot", "hello").unwrap_err();
    matches!(err, GoAppError::UnknownRole(_));
}

#[test]
fn go_message_accepts_valid_roles() {
    let app = GoApp::new("go-agent");
    for role in &["user", "assistant", "system"] {
        assert!(app.create_message(role, "content").is_ok());
    }
}
