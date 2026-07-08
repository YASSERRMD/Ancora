use crate::{
    dotnet_app::DotnetApp, go_app::GoApp, java_app::JavaApp, python_app::PythonApp,
    rust_app::RustApp, ts_app::TsApp,
};

/// All apps must reject empty input rather than panicking or returning garbage.
#[test]
fn go_app_guardrail_empty_input() {
    let app = GoApp::new("agent");
    assert!(app.run("").is_err());
}

#[test]
fn python_app_guardrail_empty_input() {
    let app = PythonApp::new("agent", "model").unwrap();
    assert!(app.run("").is_err());
}

#[test]
fn ts_app_guardrail_empty_input() {
    let app = TsApp::new("agent", "1.0").unwrap();
    assert!(app.run("").is_err());
}

#[test]
fn dotnet_app_guardrail_empty_input() {
    let app = DotnetApp::new("agent", "net8.0").unwrap();
    assert!(app.run("").is_err());
}

#[test]
fn java_app_guardrail_empty_input() {
    let app = JavaApp::new("agent", 17).unwrap();
    assert!(app.run("").is_err());
}

#[test]
fn rust_app_guardrail_empty_input() {
    let app = RustApp::new("agent", 2021).unwrap();
    assert!(app.run("").is_err());
}

/// All apps must reject invalid construction parameters.
#[test]
fn python_app_guardrail_empty_model() {
    assert!(PythonApp::new("agent", "").is_err());
}

#[test]
fn ts_app_guardrail_empty_sdk_version() {
    assert!(TsApp::new("agent", "").is_err());
}

#[test]
fn dotnet_app_guardrail_bad_framework() {
    assert!(DotnetApp::new("agent", "netstandard2.0").is_err());
}

#[test]
fn java_app_guardrail_old_version() {
    assert!(JavaApp::new("agent", 7).is_err());
}

#[test]
fn rust_app_guardrail_old_edition() {
    assert!(RustApp::new("agent", 2015).is_err());
}
