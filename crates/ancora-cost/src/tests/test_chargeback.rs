#[cfg(test)]
mod tests {
    use crate::{
        attribution::{CostAttributor, CostRecord},
        chargeback::ChargebackReport,
    };

    fn make_attributor() -> CostAttributor {
        let mut a = CostAttributor::default();
        a.record(CostRecord { tenant_id: "t1".into(), run_id: "r1".into(), model: "gpt-4o".into(), provider: "openai".into(), tool: None, tokens: 1000, cost_usd: 0.10 });
        a.record(CostRecord { tenant_id: "t1".into(), run_id: "r2".into(), model: "gpt-4o".into(), provider: "openai".into(), tool: None, tokens: 500, cost_usd: 0.05 });
        a.record(CostRecord { tenant_id: "t2".into(), run_id: "r3".into(), model: "claude-3".into(), provider: "anthropic".into(), tool: None, tokens: 800, cost_usd: 0.08 });
        a
    }

    #[test]
    fn chargeback_report_accurate() {
        let a = make_attributor();
        let report = ChargebackReport::generate(&a, 0, 86400);
        assert_eq!(report.lines.len(), 2);
        let t1 = report.lines.iter().find(|l| l.tenant_id == "t1").unwrap();
        assert!((t1.total_cost_usd - 0.15).abs() < 1e-9);
        assert_eq!(t1.total_tokens, 1500);
    }

    #[test]
    fn chargeback_total_cost_sums() {
        let a = make_attributor();
        let report = ChargebackReport::generate(&a, 0, 86400);
        assert!((report.total_cost_usd() - 0.23).abs() < 1e-9);
    }
}
