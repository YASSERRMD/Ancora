//! Network policy enforcement with egress rules, allow/deny lists, and audit logging for Ancora.
//!
//! Core types: [`NetworkRule`], [`NetworkPolicy`], [`ConnectionRequest`].
//! Evaluation: [`PolicyEvaluator`] returning [`PolicyDecision`] (Allow/Deny/NoPolicy).
//! Audit: [`NetpolAuditLog`], [`EvaluationRecord`] for tracking all evaluations.
//! Presets: [`presets`] module for common policies (allow-https-only, allow-internal-only, block-known-bad).
//! Summary: [`PolicySummary`] with rule counts and evaluation statistics.
pub mod audit;
pub mod connection;
pub mod evaluator;
pub mod policy;
pub mod presets;
pub mod rule;
pub mod summary;

pub use audit::{EvaluationRecord, NetpolAuditLog};
pub use connection::ConnectionRequest;
pub use evaluator::{PolicyDecision, PolicyEvaluator};
pub use policy::{DefaultPosture, NetworkPolicy, PolicyStore};
pub use rule::{Effect, NetworkRule, Protocol};
pub use summary::PolicySummary;

#[cfg(test)]
mod tests {
    mod test_rule_matches_host;
    mod test_rule_matches_port;
    mod test_rule_wildcard;
    mod test_rule_any_protocol;
    mod test_connection_tcp;
    mod test_connection_udp;
    mod test_policy_add_rule;
    mod test_policy_deny_by_default;
    mod test_policy_allow_by_default;
    mod test_policy_rule_sorting;
    mod test_evaluator_allow;
    mod test_evaluator_deny;
    mod test_evaluator_default_deny;
    mod test_evaluator_default_allow;
    mod test_evaluator_first_match_wins;
    mod test_audit_log_record;
    mod test_audit_log_denied;
    mod test_audit_log_allowed;
    mod test_preset_https_only;
    mod test_preset_internal_only;
    mod test_preset_block_known_bad;
    mod test_summary_rule_counts;
    mod test_summary_deny_rate;
    mod test_policy_store;
    mod test_rule_effect_variants;
    mod test_protocol_variants;
    mod test_evaluator_no_matching_rule;
    mod test_policy_allow_deny_rules;
    mod test_audit_log_all;
    mod test_connection_factory;
}
