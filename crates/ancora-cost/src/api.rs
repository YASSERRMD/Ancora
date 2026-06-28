use serde::{Deserialize, Serialize};
use crate::{attribution::CostAttributor, budget::TenantBudget};

/// Cost API response for a tenant.
#[derive(Debug, Serialize, Deserialize)]
pub struct TenantCostSummary {
    pub tenant_id: String,
    pub spent_usd: f64,
    pub budget_usd: f64,
    pub remaining_usd: f64,
    pub pct_used: f64,
}

impl TenantCostSummary {
    pub fn from_budget(budget: &TenantBudget) -> Self {
        let pct_used = if budget.hard_limit_usd > 0.0 {
            budget.spent_usd / budget.hard_limit_usd * 100.0
        } else {
            0.0
        };
        Self {
            tenant_id: budget.tenant_id.clone(),
            spent_usd: budget.spent_usd,
            budget_usd: budget.hard_limit_usd,
            remaining_usd: budget.remaining_usd(),
            pct_used,
        }
    }
}

/// Cost dashboard JSON (simplified for offline generation).
pub fn cost_dashboard(attributor: &CostAttributor) -> serde_json::Value {
    let total: f64 = attributor.all_records().iter().map(|r| r.cost_usd).sum();
    let total_tokens: u64 = attributor.all_records().iter().map(|r| r.tokens).sum();
    serde_json::json!({
        "title": "Ancora Cost Dashboard",
        "total_cost_usd": total,
        "total_tokens": total_tokens,
        "record_count": attributor.all_records().len()
    })
}

/// List all tenant cost summaries from a set of budgets.
pub fn list_tenant_summaries(budgets: &[crate::budget::TenantBudget]) -> Vec<TenantCostSummary> {
    budgets.iter().map(TenantCostSummary::from_budget).collect()
}
