use crate::{IssueKind, NetworkPolicy, PolicyValidator, RuleBuilder};
#[test]
fn validator_reports_no_rules_issue() {
    let policy = NetworkPolicy::deny_by_default("t1");
    let issues = PolicyValidator::validate(&policy);
    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0].kind, IssueKind::NoRules);
}
#[test]
fn validator_reports_duplicate_id() {
    let mut policy = NetworkPolicy::allow_by_default("t1");
    policy.add_rule(RuleBuilder::new("dup").build());
    policy.add_rule(RuleBuilder::new("dup").build());
    let issues = PolicyValidator::validate(&policy);
    assert!(issues.iter().any(|i| i.kind == IssueKind::DuplicateId));
}
#[test]
fn valid_policy_has_no_issues() {
    let mut policy = NetworkPolicy::deny_by_default("t1");
    policy.add_rule(RuleBuilder::new("r1").host("api.com").port(443).tcp().allow().build());
    assert!(PolicyValidator::is_valid(&policy));
}
