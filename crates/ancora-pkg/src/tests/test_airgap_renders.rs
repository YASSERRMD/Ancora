use crate::airgap_template::{AirgapConfig, AirgapError, AirgapTemplate, ArtifactSource};

#[test]
fn test_airgap_local_registry() {
    let config = AirgapConfig::new(
        "secure-agent",
        "2.0.0",
        ArtifactSource::LocalRegistry("registry.airgap.local:5000".to_string()),
        "/etc/ancora/license.key",
    );
    let tmpl = AirgapTemplate::render(config).expect("should render");
    assert!(tmpl.contains("secure-agent"));
    assert!(tmpl.contains("registry.airgap.local:5000"));
    assert!(tmpl.contains("local_registry"));
}

#[test]
fn test_airgap_tar_bundle() {
    let config = AirgapConfig::new(
        "bundle-agent",
        "1.5.0",
        ArtifactSource::TarBundle("/media/ancora-bundle.tar.gz".to_string()),
        "/etc/ancora/license.key",
    );
    let tmpl = AirgapTemplate::render(config).expect("should render");
    assert!(tmpl.contains("tar_bundle"));
    assert!(tmpl.contains("/media/ancora-bundle.tar.gz"));
}

#[test]
fn test_airgap_no_external_access() {
    let config = AirgapConfig::new(
        "airgap-test",
        "1.0.0",
        ArtifactSource::LocalRegistry("local:5000".to_string()),
        "/etc/ancora/license.key",
    );
    let tmpl = AirgapTemplate::render(config).expect("should render");
    assert!(tmpl.contains("external_access: false"), "Must disable external access");
}

#[test]
fn test_airgap_fips_mode() {
    let config = AirgapConfig::new(
        "fips-agent",
        "1.0.0",
        ArtifactSource::LocalRegistry("local:5000".to_string()),
        "/etc/ancora/license.key",
    );
    let tmpl = AirgapTemplate::render(config).expect("should render");
    assert!(tmpl.contains("fips_mode: true"), "FIPS mode must be enabled for air-gapped deployments");
}

#[test]
fn test_airgap_validation_empty_product() {
    let config = AirgapConfig::new(
        "",
        "1.0.0",
        ArtifactSource::LocalRegistry("local:5000".to_string()),
        "/etc/ancora/license.key",
    );
    let report = AirgapTemplate::validate(&config);
    assert!(!report.passed);
    assert!(!report.issues.is_empty());
}

#[test]
fn test_airgap_render_fails_on_invalid() {
    let config = AirgapConfig::new(
        "",
        "",
        ArtifactSource::TarBundle("/bundle.tar.gz".to_string()),
        "",
    );
    let err = AirgapTemplate::render(config).unwrap_err();
    assert!(matches!(err, AirgapError::ValidationFailed(_)));
}

#[test]
fn test_airgap_node_count() {
    let config = AirgapConfig::new(
        "cluster-agent",
        "1.0.0",
        ArtifactSource::LocalRegistry("local:5000".to_string()),
        "/etc/ancora/license.key",
    ).with_node_count(5);
    let tmpl = AirgapTemplate::render(config).expect("should render");
    assert!(tmpl.contains("node_count: 5"));
}

#[test]
fn test_artifact_source_helpers() {
    let src = ArtifactSource::LocalRegistry("http://reg".to_string());
    assert_eq!(src.registry_url(), Some("http://reg"));
    assert_eq!(src.bundle_path(), None);

    let src2 = ArtifactSource::TarBundle("/path/bundle.tar".to_string());
    assert_eq!(src2.registry_url(), None);
    assert_eq!(src2.bundle_path(), Some("/path/bundle.tar"));
}
