use crate::{
    airgap_template::{AirgapConfig, AirgapTemplate, ArtifactSource},
    compose_template::{ComposeConfig, ComposeService, ComposeTemplate},
    edge_template::{EdgeArch, EdgeConfig, EdgeTemplate},
    k8s_template::{K8sConfig, K8sTemplate},
    onprem_template::{OnPremConfig, OnPremTemplate},
    saas_template::{SaasConfig, SaasTemplate, SaasTier, SecureDefaults},
};

#[test]
fn test_saas_secure_defaults_tls13() {
    let defaults = SecureDefaults::default();
    assert_eq!(defaults.tls_min_version, "TLSv1.3");
}

#[test]
fn test_saas_secure_defaults_hsts() {
    let defaults = SecureDefaults::default();
    assert_eq!(defaults.hsts_max_age_seconds, 31_536_000);
}

#[test]
fn test_saas_secure_defaults_all_enabled() {
    let defaults = SecureDefaults::default();
    assert!(defaults.csp_enabled, "CSP must be on by default");
    assert!(defaults.rate_limiting_enabled, "rate limiting must be on");
    assert!(defaults.audit_logging_enabled, "audit logging must be on");
    assert!(defaults.mfa_required, "MFA must be required");
}

#[test]
fn test_k8s_pod_security_non_root() {
    let config = K8sConfig::new("sec-agent", "ns", "img:v1");
    assert!(config.pod_security.run_as_non_root);
    assert!(!config.pod_security.allow_privilege_escalation);
    assert!(config.pod_security.read_only_root_filesystem);
}

#[test]
fn test_k8s_network_policy_on_by_default() {
    let config = K8sConfig::new("netpol-agent", "ns", "img:v1");
    assert!(
        config.network_policy_enabled,
        "network policy must be on by default"
    );
}

#[test]
fn test_onprem_tls_required_in_output() {
    let config = OnPremConfig::new("sec-onprem", "host.local", 1);
    let tmpl = OnPremTemplate::render(config).expect("should render");
    assert!(tmpl.contains("tls: required"));
}

#[test]
fn test_airgap_fips_in_output() {
    let config = AirgapConfig::new(
        "fips-agent",
        "1.0.0",
        ArtifactSource::LocalRegistry("local:5000".to_string()),
        "/etc/ancora/license.key",
    );
    let tmpl = AirgapTemplate::render(config).expect("should render");
    assert!(tmpl.contains("fips_mode: true"));
}

#[test]
fn test_compose_no_new_privileges_default() {
    let svc = ComposeService::new("svc", "img:v1");
    assert!(svc.no_new_privileges);
    assert!(svc.read_only);
}

#[test]
fn test_edge_security_runtime_fields() {
    let config = EdgeConfig::new("edge-agent", "1.0.0", EdgeArch::Aarch64);
    let tmpl = EdgeTemplate::render(config).expect("should render");
    assert!(tmpl.runtime_config.contains("tls: required"));
    assert!(tmpl.runtime_config.contains("no_root: true"));
    assert!(tmpl.runtime_config.contains("audit_log: enabled"));
}

#[test]
fn test_all_templates_have_audit_log() {
    // SaaS
    let saas_config = SaasConfig::new("a", SaasTier::Production, "us-east-1");
    let saas_tmpl = SaasTemplate::render(saas_config).unwrap();
    assert!(saas_tmpl.rendered_yaml.contains("audit_logging: true"));

    // On-prem
    let onprem_config = OnPremConfig::new("b", "h.local", 1);
    let onprem_tmpl = OnPremTemplate::render(onprem_config).unwrap();
    assert!(onprem_tmpl.contains("audit_log"));

    // Air-gapped
    let airgap_config = AirgapConfig::new(
        "c",
        "1.0.0",
        ArtifactSource::LocalRegistry("r:5000".to_string()),
        "/lic",
    );
    let airgap_tmpl = AirgapTemplate::render(airgap_config).unwrap();
    assert!(airgap_tmpl.contains("audit_log: enabled"));

    // Edge
    let edge_config = EdgeConfig::new("d", "1.0.0", EdgeArch::X86_64);
    let edge_tmpl = EdgeTemplate::render(edge_config).unwrap();
    assert!(edge_tmpl.runtime_config.contains("audit_log: enabled"));

    // K8s (audit log is implied via operator secureDefaults)
    let k8s_config = K8sConfig::new("e", "ns", "img:v1");
    let k8s_tmpl = K8sTemplate::render(k8s_config).unwrap();
    assert!(k8s_tmpl
        .operator_cr_yaml
        .as_ref()
        .unwrap()
        .contains("secureDefaults: true"));
}
