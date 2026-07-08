use crate::selfhosted::{is_export_permitted, ResidencyError, ResidencyPolicy, SelfHostedConfig};

#[test]
fn test_unrestricted_policy_allows_any_endpoint() {
    let policy = ResidencyPolicy::Unrestricted;
    assert!(policy.check_endpoint("https://cloud.langfuse.com").is_ok());
    assert!(policy.check_endpoint("https://api.datadoghq.com").is_ok());
}

#[test]
fn test_self_hosted_policy_blocks_external() {
    let policy = ResidencyPolicy::self_hosted(vec!["http://internal.corp".to_string()]);
    let result = policy.check_endpoint("https://cloud.langfuse.com");
    assert!(result.is_err());
    matches!(
        result.unwrap_err(),
        ResidencyError::ExternalEndpointBlocked { .. }
    );
}

#[test]
fn test_self_hosted_policy_allows_matching_prefix() {
    let policy = ResidencyPolicy::self_hosted(vec!["http://internal.corp".to_string()]);
    assert!(policy.check_endpoint("http://internal.corp/otlp").is_ok());
    assert!(policy
        .check_endpoint("http://internal.corp:4317/traces")
        .is_ok());
}

#[test]
fn test_self_hosted_policy_multiple_prefixes() {
    let policy = ResidencyPolicy::self_hosted(vec![
        "http://tempo.internal".to_string(),
        "http://loki.internal".to_string(),
    ]);
    assert!(policy.check_endpoint("http://tempo.internal/push").is_ok());
    assert!(policy
        .check_endpoint("http://loki.internal/loki/api/v1/push")
        .is_ok());
    assert!(policy.check_endpoint("https://cloud.datadog.com").is_err());
}

#[test]
fn test_is_export_permitted_unrestricted() {
    let policy = ResidencyPolicy::Unrestricted;
    assert!(is_export_permitted(&policy, "https://anything.example.com"));
}

#[test]
fn test_is_export_permitted_self_hosted_blocks() {
    let policy = ResidencyPolicy::self_hosted(vec!["http://10.0.0.0".to_string()]);
    assert!(!is_export_permitted(&policy, "https://external.service.io"));
}

#[test]
fn test_selfhosted_config_validate_ok() {
    let policy = ResidencyPolicy::self_hosted(vec!["http://internal".to_string()]);
    let cfg = SelfHostedConfig::new(policy)
        .with_tempo("http://internal/tempo")
        .with_loki("http://internal/loki");
    assert!(cfg.validate().is_ok());
}

#[test]
fn test_selfhosted_config_validate_blocks_external_endpoint() {
    let policy = ResidencyPolicy::self_hosted(vec!["http://internal".to_string()]);
    let cfg = SelfHostedConfig::new(policy).with_otlp("https://cloud.provider.io/otlp");
    assert!(cfg.validate().is_err());
}

#[test]
fn test_residency_policy_is_self_hosted() {
    assert!(!ResidencyPolicy::Unrestricted.is_self_hosted());
    assert!(ResidencyPolicy::self_hosted(vec![]).is_self_hosted());
}
