use crate::compose_template::{ComposeConfig, ComposeError, ComposeService, ComposeTemplate};

#[test]
fn test_compose_basic_render() {
    let svc = ComposeService::new("agent", "ancora/agent:latest").with_port(8080, 8080);
    let config = ComposeConfig::new("my-stack").add_service(svc);
    let tmpl = ComposeTemplate::render(config).expect("should render");
    assert!(tmpl.rendered.contains("my-stack"));
    assert!(tmpl.rendered.contains("agent"));
    assert_eq!(tmpl.service_count(), 1);
}

#[test]
fn test_compose_secure_defaults() {
    let svc = ComposeService::new("secure-svc", "ancora/agent:latest");
    assert!(svc.read_only, "services should default to read_only=true");
    assert!(
        svc.no_new_privileges,
        "services should default to no_new_privileges=true"
    );
}

#[test]
fn test_compose_readonly_in_output() {
    let svc = ComposeService::new("agent", "ancora/agent:latest");
    let config = ComposeConfig::new("proj").add_service(svc);
    let tmpl = ComposeTemplate::render(config).expect("should render");
    assert!(
        tmpl.contains("read_only: true"),
        "read_only must appear in rendered output"
    );
    assert!(
        tmpl.contains("no-new-privileges:true"),
        "no-new-privileges must appear"
    );
}

#[test]
fn test_compose_internal_network() {
    let svc = ComposeService::new("agent", "ancora/agent:latest");
    let config = ComposeConfig::new("nettest").add_service(svc);
    let tmpl = ComposeTemplate::render(config).expect("should render");
    assert!(tmpl.contains("internal: true"), "network must be internal");
}

#[test]
fn test_compose_environment_vars() {
    let svc = ComposeService::new("cfg-svc", "ancora/agent:latest")
        .with_env("LOG_LEVEL", "info")
        .with_env("TLS_MODE", "required");
    let config = ComposeConfig::new("env-stack").add_service(svc);
    let tmpl = ComposeTemplate::render(config).expect("should render");
    assert!(tmpl.contains("LOG_LEVEL: info"));
    assert!(tmpl.contains("TLS_MODE: required"));
}

#[test]
fn test_compose_multiple_services() {
    let api = ComposeService::new("api", "ancora/api:latest").with_port(8080, 8080);
    let db = ComposeService::new("db", "postgres:15");
    let config = ComposeConfig::new("full-stack")
        .add_service(api)
        .add_service(db);
    let tmpl = ComposeTemplate::render(config).expect("should render");
    assert_eq!(tmpl.service_count(), 2);
}

#[test]
fn test_compose_depends_on() {
    let db = ComposeService::new("db", "postgres:15");
    let api = ComposeService::new("api", "ancora/api:latest").depends_on("db");
    let config = ComposeConfig::new("deps-stack")
        .add_service(db)
        .add_service(api);
    let tmpl = ComposeTemplate::render(config).expect("should render");
    assert!(tmpl.contains("depends_on"));
    assert!(tmpl.contains("- db"));
}

#[test]
fn test_compose_empty_project_name_fails() {
    let svc = ComposeService::new("svc", "img:latest");
    let config = ComposeConfig::new("").add_service(svc);
    let err = ComposeTemplate::render(config).unwrap_err();
    assert!(matches!(err, ComposeError::InvalidConfig(_)));
}

#[test]
fn test_compose_no_services_fails() {
    let config = ComposeConfig::new("empty-stack");
    let err = ComposeTemplate::render(config).unwrap_err();
    assert!(matches!(err, ComposeError::InvalidConfig(_)));
}

#[test]
fn test_compose_volumes() {
    let svc = ComposeService::new("svc", "img:latest");
    let config = ComposeConfig::new("vol-stack")
        .add_service(svc)
        .add_volume("ancora-data");
    let tmpl = ComposeTemplate::render(config).expect("should render");
    assert!(tmpl.contains("ancora-data"));
}
