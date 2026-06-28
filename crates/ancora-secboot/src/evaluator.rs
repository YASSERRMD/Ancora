use crate::chain::BootChain;
use crate::policy::BootPolicy;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntegrityDecision {
    Pass,
    Fail(String),
}

impl IntegrityDecision {
    pub fn is_pass(&self) -> bool { matches!(self, IntegrityDecision::Pass) }
}

pub struct IntegrityEvaluator;

impl IntegrityEvaluator {
    pub fn evaluate(policy: &BootPolicy, chain: &BootChain) -> IntegrityDecision {
        if chain.is_empty() {
            return IntegrityDecision::Fail("empty boot chain".to_string());
        }
        let present_kinds = chain.present_kinds();
        if !policy.required_kinds_met(&present_kinds) {
            let missing: Vec<_> = policy.require_kinds.difference(&present_kinds).cloned().collect();
            return IntegrityDecision::Fail(format!("missing required kinds: {:?}", missing));
        }
        for step in chain.steps() {
            if !policy.is_digest_allowed(&step.name, &step.digest) {
                return IntegrityDecision::Fail(format!("unauthorized digest for {}", step.name));
            }
        }
        IntegrityDecision::Pass
    }
}
