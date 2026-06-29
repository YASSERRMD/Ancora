use crate::policy::*;
use crate::registration::DeviceId;

#[test]
fn test_policy_update_applied() {
    let mut svc = RemotePolicyService::new();
    let mut policy = Policy::new("sec-policy-v1", 1);
    policy.add_rule("tls_required", "true");
    policy.add_rule("min_key_bits", "256");

    let id = DeviceId::new("dev-001");
    let record = svc.push_policy(&id, &policy);

    assert_eq!(record.status, PolicyUpdateStatus::Applied);
    assert!(svc.is_applied(&id, "sec-policy-v1"));
}

#[test]
fn test_policy_pushed_to_fleet() {
    let mut svc = RemotePolicyService::new();
    let policy = Policy::new("audit-policy", 2);
    let ids: Vec<DeviceId> = (0..4).map(|i| DeviceId::new(format!("d-{}", i))).collect();

    let records = svc.push_to_fleet(&ids, &policy);
    assert_eq!(records.len(), 4);
    assert_eq!(svc.applied_devices("audit-policy").len(), 4);
}

#[test]
fn test_policy_rule_lookup() {
    let mut policy = Policy::new("firewall-policy", 1);
    policy.add_rule("allow_inbound_443", "true");
    policy.add_rule("allow_inbound_80", "false");

    assert_eq!(policy.get_rule("allow_inbound_443"), Some("true"));
    assert_eq!(policy.get_rule("allow_inbound_80"), Some("false"));
    assert!(policy.get_rule("nonexistent").is_none());
}
