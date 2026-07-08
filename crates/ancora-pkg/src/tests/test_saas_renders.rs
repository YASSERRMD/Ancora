use crate::saas_template::{SaasConfig, SaasTemplate, SaasTier, TemplateError};

#[test]
fn test_saas_render_production() {
    let config = SaasConfig::new("my-saas", SaasTier::Production, "us-east-1");
    let tmpl = SaasTemplate::render(config).expect("should render");
    assert!(tmpl.rendered_yaml.contains("my-saas"));
    assert!(tmpl.rendered_yaml.contains("production"));
    assert_eq!(tmpl.config.replicas, 3);
}

#[test]
fn test_saas_render_development() {
    let config = SaasConfig::new("dev-app", SaasTier::Development, "eu-west-1");
    let tmpl = SaasTemplate::render(config).expect("should render");
    assert_eq!(tmpl.config.replicas, 1);
    assert!(tmpl.rendered_yaml.contains("development"));
}

#[test]
fn test_saas_secure_defaults_present() {
    let config = SaasConfig::new("secure-saas", SaasTier::Production, "us-west-2");
    let tmpl = SaasTemplate::render(config).expect("should render");
    assert!(
        tmpl.has_security_field("TLSv1.3"),
        "TLS version must appear"
    );
    assert!(
        tmpl.has_security_field("runAsNonRoot: true"),
        "non-root must be set"
    );
    assert!(
        tmpl.has_security_field("readOnlyRootFilesystem: true"),
        "read-only root must be set"
    );
}

#[test]
fn test_saas_empty_product_name_fails() {
    let config = SaasConfig::new("", SaasTier::Production, "us-east-1");
    let err = SaasTemplate::render(config).unwrap_err();
    assert!(matches!(err, TemplateError::InvalidConfig(_)));
}

#[test]
fn test_saas_feature_flags() {
    let config = SaasConfig::new("feature-app", SaasTier::Staging, "ap-southeast-1")
        .with_feature("beta_ui", true)
        .with_feature("legacy_api", false);
    let tmpl = SaasTemplate::render(config).expect("should render");
    assert_eq!(tmpl.config.feature_flags.get("beta_ui"), Some(&true));
    assert_eq!(tmpl.config.feature_flags.get("legacy_api"), Some(&false));
}

#[test]
fn test_saas_staging_replicas() {
    let config = SaasConfig::new("staging-app", SaasTier::Staging, "us-east-1");
    let tmpl = SaasTemplate::render(config).expect("should render");
    assert_eq!(tmpl.config.replicas, 2);
}

#[test]
fn test_saas_audit_logging_enabled() {
    let config = SaasConfig::new("audit-app", SaasTier::Production, "us-east-1");
    assert!(config.secure_defaults.audit_logging_enabled);
    assert!(config.secure_defaults.mfa_required);
}
