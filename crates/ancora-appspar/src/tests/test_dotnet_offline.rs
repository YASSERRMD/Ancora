use crate::dotnet_app::{DotnetApp, DotnetAppError};

#[test]
fn dotnet_app_runs_offline() {
    let app = DotnetApp::new("dotnet-agent", "net8.0").unwrap();
    let trace = app.run("hello dotnet").unwrap();
    assert_eq!(trace.messages.len(), 2);
    assert_eq!(trace.messages[0].role, "user");
    assert_eq!(trace.messages[1].role, "assistant");
    assert_eq!(trace.framework, "net8.0");
    assert!(trace.trace_id.starts_with("dotnet-trace-"));
}

#[test]
fn dotnet_app_rejects_empty_content() {
    let app = DotnetApp::new("dotnet-agent", "net8.0").unwrap();
    let err = app.run("").unwrap_err();
    assert_eq!(err, DotnetAppError::EmptyContent);
}

#[test]
fn dotnet_app_rejects_unsupported_framework() {
    let err = DotnetApp::new("dotnet-agent", "net4.0").unwrap_err();
    matches!(err, DotnetAppError::UnsupportedFramework(_));
}

#[test]
fn dotnet_app_supports_net9() {
    let app = DotnetApp::new("dotnet-agent", "net9.0").unwrap();
    let trace = app.run("test").unwrap();
    assert_eq!(trace.framework, "net9.0");
}
