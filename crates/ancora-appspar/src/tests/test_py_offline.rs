use crate::python_app::{PythonApp, PythonAppError};

#[test]
fn python_app_runs_offline() {
    let app = PythonApp::new("py-agent", "claude-opus-4-5").unwrap();
    let trace = app.run("hello python").unwrap();
    assert_eq!(trace.messages.len(), 2);
    assert_eq!(trace.messages[0].role, "user");
    assert_eq!(trace.messages[1].role, "assistant");
    assert!(trace.trace_id.starts_with("py-trace-"));
}

#[test]
fn python_app_counts_tokens() {
    let app = PythonApp::new("py-agent", "claude-opus-4-5").unwrap();
    let trace = app.run("one two three").unwrap();
    assert_eq!(trace.input_tokens, 3);
    assert_eq!(trace.output_tokens, 6);
}

#[test]
fn python_app_rejects_empty_content() {
    let app = PythonApp::new("py-agent", "claude-opus-4-5").unwrap();
    let err = app.run("").unwrap_err();
    assert_eq!(err, PythonAppError::EmptyContent);
}

#[test]
fn python_app_rejects_empty_model() {
    let err = PythonApp::new("py-agent", "").unwrap_err();
    matches!(err, PythonAppError::InvalidModel(_));
}
