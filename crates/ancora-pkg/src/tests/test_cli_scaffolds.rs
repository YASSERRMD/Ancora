use crate::cli::{CliError, PackagingCli, ScaffoldArgs, ScaffoldKind};

#[test]
fn test_cli_scaffold_saas() {
    let args = ScaffoldArgs::new(ScaffoldKind::Saas, "my-saas")
        .with_output("/tmp/test-output")
        .with_extra("region", "eu-west-1");
    let out = PackagingCli::scaffold(args).expect("should scaffold");
    assert_eq!(out.kind, ScaffoldKind::Saas);
    assert!(!out.files.is_empty());
    assert!(out.find_file("deployment.yaml").is_some());
    assert!(out.summary.contains("my-saas"));
}

#[test]
fn test_cli_scaffold_onprem() {
    let args = ScaffoldArgs::new(ScaffoldKind::OnPrem, "my-appliance")
        .with_extra("hostname", "ancora.corp.local");
    let out = PackagingCli::scaffold(args).expect("should scaffold");
    assert_eq!(out.kind, ScaffoldKind::OnPrem);
    assert!(out.find_file("appliance.yaml").is_some());
}

#[test]
fn test_cli_scaffold_airgap() {
    let args = ScaffoldArgs::new(ScaffoldKind::Airgap, "my-airgap")
        .with_extra("registry", "registry.corp.local:5000");
    let out = PackagingCli::scaffold(args).expect("should scaffold");
    assert_eq!(out.kind, ScaffoldKind::Airgap);
    assert!(out.find_file("airgap.yaml").is_some());
}

#[test]
fn test_cli_scaffold_compose() {
    let args = ScaffoldArgs::new(ScaffoldKind::Compose, "my-compose");
    let out = PackagingCli::scaffold(args).expect("should scaffold");
    assert_eq!(out.kind, ScaffoldKind::Compose);
    assert!(out.find_file("docker-compose.yml").is_some());
}

#[test]
fn test_cli_scaffold_kubernetes() {
    let args = ScaffoldArgs::new(ScaffoldKind::Kubernetes, "my-k8s")
        .with_extra("namespace", "production")
        .with_extra("image", "ancora/agent:v2");
    let out = PackagingCli::scaffold(args).expect("should scaffold");
    assert_eq!(out.kind, ScaffoldKind::Kubernetes);
    assert!(out.find_file("manifests.yaml").is_some());
}

#[test]
fn test_cli_scaffold_edge() {
    let args = ScaffoldArgs::new(ScaffoldKind::Edge, "my-edge");
    let out = PackagingCli::scaffold(args).expect("should scaffold");
    assert_eq!(out.kind, ScaffoldKind::Edge);
    assert_eq!(out.file_count(), 2, "edge produces build + runtime files");
    assert!(out.find_file("build.yaml").is_some());
    assert!(out.find_file("runtime.yaml").is_some());
}

#[test]
fn test_cli_scaffold_whitelabel() {
    let args = ScaffoldArgs::new(ScaffoldKind::Whitelabel, "AcmeBrand")
        .with_extra("domain", "acme.example.com");
    let out = PackagingCli::scaffold(args).expect("should scaffold");
    assert_eq!(out.kind, ScaffoldKind::Whitelabel);
    assert!(out.find_file("whitelabel.yaml").is_some());
}

#[test]
fn test_cli_scaffold_tenant() {
    let args = ScaffoldArgs::new(ScaffoldKind::TenantOnboard, "Acme Inc")
        .with_extra("admin_email", "admin@acme.com");
    let out = PackagingCli::scaffold(args).expect("should scaffold");
    assert_eq!(out.kind, ScaffoldKind::TenantOnboard);
    assert!(out.find_file("tenant.yaml").is_some());
}

#[test]
fn test_cli_empty_product_name_fails() {
    let args = ScaffoldArgs::new(ScaffoldKind::Saas, "");
    let err = PackagingCli::scaffold(args).unwrap_err();
    assert!(matches!(err, CliError::MissingArg(_)));
}

#[test]
fn test_scaffold_kind_from_str() {
    assert_eq!(ScaffoldKind::parse_str("saas"), Some(ScaffoldKind::Saas));
    assert_eq!(
        ScaffoldKind::parse_str("onprem"),
        Some(ScaffoldKind::OnPrem)
    );
    assert_eq!(
        ScaffoldKind::parse_str("airgap"),
        Some(ScaffoldKind::Airgap)
    );
    assert_eq!(
        ScaffoldKind::parse_str("compose"),
        Some(ScaffoldKind::Compose)
    );
    assert_eq!(
        ScaffoldKind::parse_str("k8s"),
        Some(ScaffoldKind::Kubernetes)
    );
    assert_eq!(
        ScaffoldKind::parse_str("kubernetes"),
        Some(ScaffoldKind::Kubernetes)
    );
    assert_eq!(ScaffoldKind::parse_str("edge"), Some(ScaffoldKind::Edge));
    assert_eq!(
        ScaffoldKind::parse_str("whitelabel"),
        Some(ScaffoldKind::Whitelabel)
    );
    assert_eq!(
        ScaffoldKind::parse_str("tenant"),
        Some(ScaffoldKind::TenantOnboard)
    );
    assert_eq!(ScaffoldKind::parse_str("unknown"), None);
}
