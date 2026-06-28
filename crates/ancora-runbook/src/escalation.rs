use crate::incident::Severity;

#[derive(Debug, Clone)]
pub struct EscalationTier {
    pub tier: u32,
    pub contact: String,
    pub wait_secs: u64,
}

pub struct EscalationPolicy {
    pub name: String,
    tiers: Vec<EscalationTier>,
}

impl EscalationPolicy {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string(), tiers: vec![] }
    }

    pub fn add_tier(mut self, contact: &str, wait_secs: u64) -> Self {
        let tier = self.tiers.len() as u32 + 1;
        self.tiers.push(EscalationTier { tier, contact: contact.to_string(), wait_secs });
        self
    }

    /// Returns the highest-priority tier whose wait_secs threshold has been reached.
    /// wait_secs is the minimum elapsed time from incident open before this tier activates.
    pub fn tier_at_elapsed(&self, elapsed_secs: u64) -> Option<&EscalationTier> {
        let mut current = None;
        for t in &self.tiers {
            if elapsed_secs >= t.wait_secs {
                current = Some(t);
            }
        }
        current
    }

    pub fn tier_count(&self) -> usize {
        self.tiers.len()
    }
}

pub fn default_policy_for(severity: &Severity) -> EscalationPolicy {
    match severity {
        Severity::P1 => EscalationPolicy::new("p1-policy")
            .add_tier("on-call-primary", 0)
            .add_tier("on-call-secondary", 300)
            .add_tier("engineering-manager", 900),
        Severity::P2 => EscalationPolicy::new("p2-policy")
            .add_tier("on-call-primary", 0)
            .add_tier("on-call-secondary", 900),
        Severity::P3 | Severity::P4 => EscalationPolicy::new("p3p4-policy")
            .add_tier("on-call-primary", 0),
    }
}
