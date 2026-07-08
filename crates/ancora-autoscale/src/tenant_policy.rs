use crate::bounds::TenantCap;
use crate::decision::ScaleDecision;
use crate::metrics::TenantMetrics;
use std::collections::HashMap;

/// Applies per-tenant caps on top of global scaling decisions.
pub struct TenantPolicyEngine {
    caps: HashMap<String, usize>,
    current_workers: HashMap<String, usize>,
}

impl TenantPolicyEngine {
    pub fn new() -> Self {
        TenantPolicyEngine {
            caps: HashMap::new(),
            current_workers: HashMap::new(),
        }
    }

    pub fn set_cap(&mut self, cap: TenantCap) {
        self.caps.insert(cap.tenant_id, cap.max_workers);
    }

    pub fn set_workers(&mut self, tenant_id: &str, count: usize) {
        self.current_workers.insert(tenant_id.to_string(), count);
    }

    /// Evaluate a global scale decision and clamp to the tenant's cap.
    pub fn apply(&self, tenant_id: &str, global: ScaleDecision) -> ScaleDecision {
        let cap = match self.caps.get(tenant_id) {
            Some(&c) => c,
            None => return global,
        };
        let current = self.current_workers.get(tenant_id).copied().unwrap_or(0);
        match global {
            ScaleDecision::ScaleUp { by } => {
                let desired = (current + by).min(cap);
                if desired > current {
                    ScaleDecision::ScaleUp {
                        by: desired - current,
                    }
                } else {
                    ScaleDecision::NoOp {
                        reason: format!("tenant {} at cap {}", tenant_id, cap),
                    }
                }
            }
            other => other,
        }
    }
}

impl Default for TenantPolicyEngine {
    fn default() -> Self {
        Self::new()
    }
}
