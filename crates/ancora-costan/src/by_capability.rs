/// Cost breakdown by capability: planner, reflection, routing, etc.
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Capability {
    Planner,
    Reflection,
    Routing,
    Reasoning,
    Generation,
    Retrieval,
    Other(String),
}

impl Capability {
    pub fn as_str(&self) -> &str {
        match self {
            Capability::Planner => "planner",
            Capability::Reflection => "reflection",
            Capability::Routing => "routing",
            Capability::Reasoning => "reasoning",
            Capability::Generation => "generation",
            Capability::Retrieval => "retrieval",
            Capability::Other(s) => s.as_str(),
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "planner" => Capability::Planner,
            "reflection" => Capability::Reflection,
            "routing" => Capability::Routing,
            "reasoning" => Capability::Reasoning,
            "generation" => Capability::Generation,
            "retrieval" => Capability::Retrieval,
            other => Capability::Other(other.to_string()),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct CapabilityCostBreakdown {
    costs: HashMap<Capability, f64>,
    calls: HashMap<Capability, u64>,
}

impl CapabilityCostBreakdown {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record(&mut self, capability: Capability, cost_usd: f64) {
        *self.costs.entry(capability.clone()).or_insert(0.0) += cost_usd;
        *self.calls.entry(capability).or_insert(0) += 1;
    }

    pub fn cost_for(&self, capability: &Capability) -> f64 {
        self.costs.get(capability).copied().unwrap_or(0.0)
    }

    pub fn calls_for(&self, capability: &Capability) -> u64 {
        self.calls.get(capability).copied().unwrap_or(0)
    }

    pub fn total_cost(&self) -> f64 {
        self.costs.values().sum()
    }

    pub fn top_capabilities(&self) -> Vec<(String, f64)> {
        let mut v: Vec<(String, f64)> = self
            .costs
            .iter()
            .map(|(k, v)| (k.as_str().to_string(), *v))
            .collect();
        v.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        v
    }

    pub fn fraction_for(&self, capability: &Capability) -> f64 {
        let total = self.total_cost();
        if total == 0.0 {
            return 0.0;
        }
        self.cost_for(capability) / total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capability_fraction_correct() {
        let mut b = CapabilityCostBreakdown::new();
        b.record(Capability::Planner, 3.0);
        b.record(Capability::Routing, 1.0);
        let frac = b.fraction_for(&Capability::Planner);
        assert!((frac - 0.75).abs() < 1e-9);
    }
}
