use crate::onprem_template::{
    NetworkIsolation, OnPremConfig, OnPremError, OnPremTemplate, SecretBackend,
};

#[test]
fn test_onprem_render_basic() {
    let config = OnPremConfig::new("my-appliance", "ancora.corp.local", 3);
    let tmpl = OnPremTemplate::render(config).expect("should render");
    assert!(tmpl.rendered.contains("my-appliance"));
    assert!(tmpl.rendered.contains("ancora.corp.local"));
    assert!(tmpl.rendered.contains("node_count: 3"));
}

#[test]
fn test_onprem_secure_fields_present() {
    let config = OnPremConfig::new("secure-onprem", "host.internal", 1);
    let tmpl = OnPremTemplate::render(config).expect("should render");
    assert!(tmpl.contains("tls: required"), "TLS must be required");
    assert!(tmpl.contains("mtls_internal: true"), "mTLS must be enabled");
    assert!(
        tmpl.contains("firewall: enabled"),
        "Firewall must be enabled"
    );
    assert!(tmpl.contains("audit_log"), "Audit log must be present");
}

#[test]
fn test_onprem_air_gapped_isolation() {
    let config = OnPremConfig::new("agap-appliance", "agap.local", 2)
        .with_isolation(NetworkIsolation::AirGapped);
    let tmpl = OnPremTemplate::render(config).expect("should render");
    assert!(tmpl.contains("air-gapped"));
}

#[test]
fn test_onprem_hsm_backend() {
    let config =
        OnPremConfig::new("hsm-appliance", "hsm.local", 1).with_secret_backend(SecretBackend::Hsm);
    let tmpl = OnPremTemplate::render(config).expect("should render");
    assert!(tmpl.contains("hsm"));
}

#[test]
fn test_onprem_empty_product_fails() {
    let config = OnPremConfig::new("", "host.local", 1);
    let err = OnPremTemplate::render(config).unwrap_err();
    assert!(matches!(err, OnPremError::InvalidConfig(_)));
}

#[test]
fn test_onprem_zero_nodes_fails() {
    let config = OnPremConfig::new("test", "host.local", 0);
    let err = OnPremTemplate::render(config).unwrap_err();
    assert!(matches!(err, OnPremError::InvalidConfig(_)));
}

#[test]
fn test_onprem_custom_data_path() {
    let config =
        OnPremConfig::new("custom-data", "host.local", 1).with_data_path("/mnt/data/ancora");
    assert_eq!(config.data_path, "/mnt/data/ancora");
    let tmpl = OnPremTemplate::render(config).expect("should render");
    assert!(tmpl.contains("/mnt/data/ancora"));
}
