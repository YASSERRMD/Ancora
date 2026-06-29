use crate::network::{
    evaluate_access, EgressPolicy, NetworkAuditLog, NetworkConfig, Protocol,
};

#[test]
fn test_no_external_egress_by_default() {
    let cfg = NetworkConfig::default();
    assert!(cfg.is_egress_blocked());
    assert!(!cfg.host_reachable("8.8.8.8"));
}

#[test]
fn test_loopback_blocked_when_external_denied() {
    let cfg = NetworkConfig::default();
    // even loopback goes through the host_reachable check for TCP
    let attempt = evaluate_access(&cfg, "8.8.8.8", 443, Protocol::Tcp);
    assert!(!attempt.allowed);
}

#[test]
fn test_unix_socket_allowed() {
    let cfg = NetworkConfig::default();
    let attempt = evaluate_access(&cfg, "/run/ancora/agent.sock", 0, Protocol::Unix);
    assert!(attempt.allowed);
}

#[test]
fn test_unix_socket_not_in_allowlist_blocked() {
    let cfg = NetworkConfig::default();
    let attempt = evaluate_access(&cfg, "/tmp/other.sock", 0, Protocol::Unix);
    assert!(!attempt.allowed);
}

#[test]
fn test_allow_list_policy() {
    let cfg = NetworkConfig::default()
        .with_egress(EgressPolicy::AllowList(vec!["api.internal".to_string()]));
    assert!(cfg.host_reachable("api.internal"));
    assert!(!cfg.host_reachable("evil.com"));
}

#[test]
fn test_fully_offline_no_dns() {
    let cfg = NetworkConfig::default();
    assert!(!cfg.allow_dns);
}

#[test]
fn test_audit_log_all_blocked() {
    let cfg = NetworkConfig::default();
    let mut log = NetworkAuditLog::new();
    for host in &["1.1.1.1", "8.8.8.8", "example.com"] {
        let a = evaluate_access(&cfg, host, 443, Protocol::Tcp);
        log.record(a);
    }
    assert!(log.all_blocked());
    assert_eq!(log.allowed_count(), 0);
    assert_eq!(log.blocked_count(), 3);
}

#[test]
fn test_egress_policy_display() {
    assert_eq!(EgressPolicy::DenyAll.to_string(), "deny-all");
    assert_eq!(EgressPolicy::AllowAll.to_string(), "allow-all");
}
