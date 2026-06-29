use crate::config::{profiles, HeadlessConfig};

#[test]
fn test_config_default_valid() {
    let cfg = HeadlessConfig::default();
    assert!(cfg.validate().is_ok());
}

#[test]
fn test_config_json_roundtrip() {
    let cfg = HeadlessConfig::new("test")
        .with_socket("/run/ancora/agent.sock")
        .with_model("/opt/models/agent-q4.gguf");
    let json = cfg.to_json().unwrap();
    let restored = HeadlessConfig::from_json(&json).unwrap();
    assert_eq!(restored.profile, "test");
    assert_eq!(restored.model_paths.len(), 1);
}

#[test]
fn test_config_invalid_cpu() {
    let cfg = HeadlessConfig { cgroup_cpu_percent: 0, ..HeadlessConfig::default() };
    assert!(cfg.validate().is_err());
}

#[test]
fn test_config_invalid_memory() {
    let cfg = HeadlessConfig { cgroup_memory_mb: 32, ..HeadlessConfig::default() };
    assert!(cfg.validate().is_err());
}

#[test]
fn test_profile_minimal_deny_network() {
    let cfg = profiles::minimal();
    assert!(cfg.deny_external_network);
    assert!(cfg.cgroup_memory_mb <= 512);
}

#[test]
fn test_profile_dev_allows_network() {
    let cfg = profiles::dev();
    assert!(!cfg.deny_external_network);
}

#[test]
fn test_profile_standard_defaults() {
    let cfg = profiles::standard();
    assert_eq!(cfg.profile, "standard");
    assert!(cfg.deny_external_network);
}
