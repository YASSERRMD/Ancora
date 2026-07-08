use crate::connection::ConnectionRequest;
use crate::policy::{DefaultPosture, NetworkPolicy};
use crate::rule::Effect;

#[derive(Debug, PartialEq, Eq)]
pub enum PolicyDecision {
    Allow,
    Deny(String),
    NoPolicy,
}

pub struct PolicyEvaluator;

impl PolicyEvaluator {
    pub fn evaluate(policy: &NetworkPolicy, req: &ConnectionRequest) -> PolicyDecision {
        for rule in &policy.rules {
            if rule.matches(&req.destination_host, req.destination_port, &req.protocol) {
                return match rule.effect {
                    Effect::Allow => PolicyDecision::Allow,
                    Effect::Deny => PolicyDecision::Deny(format!(
                        "denied by rule '{}': {}",
                        rule.id, rule.description
                    )),
                };
            }
        }
        match policy.default_posture {
            DefaultPosture::AllowAll => PolicyDecision::Allow,
            DefaultPosture::DenyAll => {
                PolicyDecision::Deny("denied by default posture".to_string())
            }
        }
    }

    pub fn is_allowed(policy: &NetworkPolicy, req: &ConnectionRequest) -> bool {
        Self::evaluate(policy, req) == PolicyDecision::Allow
    }
}
