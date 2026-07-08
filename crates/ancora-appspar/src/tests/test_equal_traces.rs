use crate::{
    dotnet_app::DotnetApp, go_app::GoApp, java_app::JavaApp, python_app::PythonApp,
    rust_app::RustApp, ts_app::TsApp,
};

/// All apps must produce exactly two messages (user + assistant) for any input.
#[test]
fn all_apps_produce_two_messages() {
    let input = "trace parity test";

    let go = GoApp::new("agent");
    assert_eq!(go.run(input).unwrap().messages.len(), 2);

    let py = PythonApp::new("agent", "model").unwrap();
    assert_eq!(py.run(input).unwrap().messages.len(), 2);

    let ts = TsApp::new("agent", "1.0.0").unwrap();
    assert_eq!(ts.run(input).unwrap().messages.len(), 2);

    let dn = DotnetApp::new("agent", "net8.0").unwrap();
    assert_eq!(dn.run(input).unwrap().messages.len(), 2);

    let jv = JavaApp::new("agent", 17).unwrap();
    assert_eq!(jv.run(input).unwrap().messages.len(), 2);

    let rs = RustApp::new("agent", 2021).unwrap();
    assert_eq!(rs.run(input).unwrap().messages.len(), 2);
}

/// First message role is always "user".
#[test]
fn all_apps_first_message_is_user() {
    let input = "user msg";

    let go = GoApp::new("agent");
    assert_eq!(go.run(input).unwrap().messages[0].role, "user");

    let py = PythonApp::new("agent", "model").unwrap();
    assert_eq!(py.run(input).unwrap().messages[0].role, "user");

    let ts = TsApp::new("agent", "1.0.0").unwrap();
    assert_eq!(ts.run(input).unwrap().messages[0].role, "user");

    let dn = DotnetApp::new("agent", "net8.0").unwrap();
    assert_eq!(dn.run(input).unwrap().messages[0].role, "user");

    let jv = JavaApp::new("agent", 17).unwrap();
    assert_eq!(jv.run(input).unwrap().messages[0].role, "user");

    let rs = RustApp::new("agent", 2021).unwrap();
    assert_eq!(rs.run(input).unwrap().messages[0].role, "user");
}

/// Second message role is always "assistant".
#[test]
fn all_apps_second_message_is_assistant() {
    let input = "assistant msg";

    let go = GoApp::new("agent");
    assert_eq!(go.run(input).unwrap().messages[1].role, "assistant");

    let py = PythonApp::new("agent", "model").unwrap();
    assert_eq!(py.run(input).unwrap().messages[1].role, "assistant");

    let ts = TsApp::new("agent", "1.0.0").unwrap();
    assert_eq!(ts.run(input).unwrap().messages[1].role, "assistant");

    let dn = DotnetApp::new("agent", "net8.0").unwrap();
    assert_eq!(dn.run(input).unwrap().messages[1].role, "assistant");

    let jv = JavaApp::new("agent", 17).unwrap();
    assert_eq!(jv.run(input).unwrap().messages[1].role, "assistant");

    let rs = RustApp::new("agent", 2021).unwrap();
    assert_eq!(rs.run(input).unwrap().messages[1].role, "assistant");
}

/// All trace IDs are unique across languages for the same input.
#[test]
fn trace_ids_are_unique_per_language() {
    let input = "unique";

    let ids = vec![
        GoApp::new("a").run(input).unwrap().trace_id,
        PythonApp::new("a", "m")
            .unwrap()
            .run(input)
            .unwrap()
            .trace_id,
        TsApp::new("a", "1.0").unwrap().run(input).unwrap().trace_id,
        DotnetApp::new("a", "net8.0")
            .unwrap()
            .run(input)
            .unwrap()
            .trace_id,
        JavaApp::new("a", 17).unwrap().run(input).unwrap().trace_id,
        RustApp::new("a", 2021)
            .unwrap()
            .run(input)
            .unwrap()
            .trace_id,
    ];

    let unique: std::collections::HashSet<_> = ids.iter().collect();
    assert_eq!(
        unique.len(),
        ids.len(),
        "trace IDs must be unique per language"
    );
}
