#[cfg(test)]
mod tests {
    use crate::{
        api::{cost_dashboard, TenantCostSummary},
        attribution::{CostAttributor, CostRecord},
        budget::{BudgetPeriod, TenantBudget},
    };

    #[test]
    fn cost_api_returns_correct_figures() {
        let b = TenantBudget::new("t1", 100.0, 0.8, BudgetPeriod::Monthly, 0);
        // Simulate spending
        let mut b2 = b.clone();
        b2.record_spend(30.0).ok();
        let summary = TenantCostSummary::from_budget(&b2);
        assert!((summary.spent_usd - 30.0).abs() < 1e-9);
        assert!((summary.remaining_usd - 70.0).abs() < 1e-9);
        assert!((summary.pct_used - 30.0).abs() < 1e-9);
    }

    #[test]
    fn cost_dashboard_json_valid() {
        let mut a = CostAttributor::default();
        a.record(CostRecord { tenant_id: "t1".into(), run_id: "r1".into(), model: "gpt-4o".into(), provider: "openai".into(), tool: None, tokens: 100, cost_usd: 0.01 });
        let d = cost_dashboard(&a);
        assert_eq!(d["title"], "Ancora Cost Dashboard");
        assert_eq!(d["record_count"], 1);
    }
}
