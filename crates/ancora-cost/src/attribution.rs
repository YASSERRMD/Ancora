use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Cost attribution record for a single run step.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CostRecord {
    pub tenant_id: String,
    pub run_id: String,
    pub model: String,
    pub provider: String,
    pub tool: Option<String>,
    pub tokens: u64,
    pub cost_usd: f64,
}

/// Aggregate cost attribution store.
#[derive(Default)]
pub struct CostAttributor {
    records: Vec<CostRecord>,
}

impl CostAttributor {
    pub fn record(&mut self, rec: CostRecord) {
        self.records.push(rec);
    }

    /// Total cost by model.
    pub fn total_by_model(&self, model: &str) -> f64 {
        self.records
            .iter()
            .filter(|r| r.model == model)
            .map(|r| r.cost_usd)
            .sum()
    }

    /// Total cost by provider.
    pub fn total_by_provider(&self, provider: &str) -> f64 {
        self.records
            .iter()
            .filter(|r| r.provider == provider)
            .map(|r| r.cost_usd)
            .sum()
    }

    /// Total cost by tool.
    pub fn total_by_tool(&self, tool: &str) -> f64 {
        self.records
            .iter()
            .filter(|r| r.tool.as_deref() == Some(tool))
            .map(|r| r.cost_usd)
            .sum()
    }

    /// Total cost by tenant.
    pub fn total_by_tenant(&self, tenant: &str) -> f64 {
        self.records
            .iter()
            .filter(|r| r.tenant_id == tenant)
            .map(|r| r.cost_usd)
            .sum()
    }

    pub fn all_records(&self) -> &[CostRecord] {
        &self.records
    }
}
