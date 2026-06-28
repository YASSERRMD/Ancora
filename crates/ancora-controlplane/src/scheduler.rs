use crate::model::{RunId, RunPriority};
use std::collections::HashMap;

/// Fair-scheduling selector: assigns run quota per tenant proportionally.
pub struct FairScheduler {
    weights: HashMap<String, u32>,
    served_counts: HashMap<String, u64>,
}

impl FairScheduler {
    pub fn new() -> Self {
        FairScheduler {
            weights: HashMap::new(),
            served_counts: HashMap::new(),
        }
    }

    pub fn set_weight(&mut self, tenant_id: &str, weight: u32) {
        self.weights.insert(tenant_id.to_string(), weight);
    }

    /// Given a list of (tenant_id, run_id, priority) candidates, pick the
    /// run that best satisfies fair share. Higher priority always wins within
    /// the same slot.
    pub fn pick<'a>(
        &mut self,
        candidates: &'a [(String, RunId, RunPriority)],
    ) -> Option<&'a RunId> {
        if candidates.is_empty() {
            return None;
        }

        // Separate critical priority - always served first
        let critical: Vec<_> = candidates
            .iter()
            .filter(|(_, _, p)| *p == RunPriority::Critical)
            .collect();
        if !critical.is_empty() {
            return Some(&critical[0].1);
        }

        // Fair-share: pick tenant with the lowest served/weight ratio
        let best = candidates.iter().min_by(|(t1, _, p1), (t2, _, p2)| {
            let ratio1 = self.fair_ratio(t1);
            let ratio2 = self.fair_ratio(t2);
            // Lower ratio = underserved, but higher priority wins ties
            ratio1
                .partial_cmp(&ratio2)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(p2.cmp(p1))
        });

        best.map(|(_, rid, _)| rid)
    }

    fn fair_ratio(&self, tenant_id: &str) -> f64 {
        let w = *self.weights.get(tenant_id).unwrap_or(&1) as f64;
        let served = *self.served_counts.get(tenant_id).unwrap_or(&0) as f64;
        served / w
    }

    pub fn record_served(&mut self, tenant_id: &str) {
        *self.served_counts.entry(tenant_id.to_string()).or_insert(0) += 1;
    }
}

impl Default for FairScheduler {
    fn default() -> Self {
        Self::new()
    }
}
