use std::collections::HashMap;
use crate::tenant::TenantId;

/// In-memory cost ledger scoped per tenant.
#[derive(Default)]
pub struct CostLedger {
    totals: HashMap<TenantId, f64>,
}

impl CostLedger {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record(&mut self, tenant_id: &TenantId, amount: f64) {
        *self.totals.entry(tenant_id.clone()).or_insert(0.0) += amount;
    }

    pub fn total_for(&self, tenant_id: &TenantId) -> f64 {
        *self.totals.get(tenant_id).unwrap_or(&0.0)
    }

    pub fn all(&self) -> &HashMap<TenantId, f64> {
        &self.totals
    }
}
