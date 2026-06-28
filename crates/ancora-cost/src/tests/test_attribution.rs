#[cfg(test)]
mod tests {
    use crate::attribution::{CostAttributor, CostRecord};

    fn rec(model: &str, provider: &str, tool: Option<&str>, tokens: u64, cost: f64, tenant: &str) -> CostRecord {
        CostRecord {
            tenant_id: tenant.into(),
            run_id: "run-1".into(),
            model: model.into(),
            provider: provider.into(),
            tool: tool.map(|t| t.into()),
            tokens,
            cost_usd: cost,
        }
    }

    #[test]
    fn attribution_sums_correctly_by_model() {
        let mut a = CostAttributor::default();
        a.record(rec("gpt-4o", "openai", None, 100, 0.01, "t1"));
        a.record(rec("gpt-4o", "openai", None, 200, 0.02, "t1"));
        a.record(rec("claude-3", "anthropic", None, 100, 0.015, "t1"));
        assert!((a.total_by_model("gpt-4o") - 0.03).abs() < 1e-9);
        assert!((a.total_by_model("claude-3") - 0.015).abs() < 1e-9);
    }

    #[test]
    fn attribution_sums_by_provider() {
        let mut a = CostAttributor::default();
        a.record(rec("gpt-4o", "openai", None, 100, 0.01, "t1"));
        a.record(rec("claude-3", "anthropic", None, 100, 0.015, "t1"));
        assert!((a.total_by_provider("openai") - 0.01).abs() < 1e-9);
    }

    #[test]
    fn attribution_sums_by_tool() {
        let mut a = CostAttributor::default();
        a.record(rec("gpt-4o", "openai", Some("web_search"), 100, 0.01, "t1"));
        a.record(rec("gpt-4o", "openai", Some("web_search"), 200, 0.02, "t1"));
        a.record(rec("gpt-4o", "openai", None, 100, 0.005, "t1"));
        assert!((a.total_by_tool("web_search") - 0.03).abs() < 1e-9);
    }

    #[test]
    fn attribution_sums_by_tenant() {
        let mut a = CostAttributor::default();
        a.record(rec("gpt-4o", "openai", None, 100, 0.01, "t1"));
        a.record(rec("gpt-4o", "openai", None, 100, 0.02, "t2"));
        assert!((a.total_by_tenant("t1") - 0.01).abs() < 1e-9);
        assert!((a.total_by_tenant("t2") - 0.02).abs() < 1e-9);
    }
}
