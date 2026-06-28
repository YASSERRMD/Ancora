use serde::{Deserialize, Serialize};
use crate::attribution::CostAttributor;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChargebackLine {
    pub tenant_id: String,
    pub total_tokens: u64,
    pub total_cost_usd: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChargebackReport {
    pub period_start_secs: u64,
    pub period_end_secs: u64,
    pub lines: Vec<ChargebackLine>,
}

impl ChargebackReport {
    pub fn generate(
        attributor: &CostAttributor,
        period_start_secs: u64,
        period_end_secs: u64,
    ) -> Self {
        let mut by_tenant: std::collections::HashMap<String, (u64, f64)> =
            std::collections::HashMap::new();
        for rec in attributor.all_records() {
            let entry = by_tenant.entry(rec.tenant_id.clone()).or_default();
            entry.0 += rec.tokens;
            entry.1 += rec.cost_usd;
        }
        let mut lines: Vec<ChargebackLine> = by_tenant
            .into_iter()
            .map(|(tenant_id, (tokens, cost))| ChargebackLine {
                tenant_id,
                total_tokens: tokens,
                total_cost_usd: cost,
            })
            .collect();
        lines.sort_by(|a, b| a.tenant_id.cmp(&b.tenant_id));
        Self { period_start_secs, period_end_secs, lines }
    }

    pub fn total_cost_usd(&self) -> f64 {
        self.lines.iter().map(|l| l.total_cost_usd).sum()
    }
}
