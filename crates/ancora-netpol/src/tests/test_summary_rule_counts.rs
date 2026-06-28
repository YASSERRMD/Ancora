use crate::{Effect, NetpolAuditLog, NetworkPolicy, NetworkRule, PolicySummary, Protocol};
#[test]
fn summary_counts_allow_and_deny_rules() {
    let mut policy = NetworkPolicy::deny_by_default("t1");
    policy.add_rule(NetworkRule::new("a1", "*", Some(443), Protocol::Tcp, Effect::Allow, 100));
    policy.add_rule(NetworkRule::new("d1", "bad.com", None, Protocol::Any, Effect::Deny, 10));
    let log = NetpolAuditLog::new();
    let summary = PolicySummary::from(&policy, &log);
    assert_eq!(summary.allow_rules, 1);
    assert_eq!(summary.deny_rules, 1);
}
