#[cfg(test)]
mod tests {
    use crate::{
        budget::{BudgetPeriod, ProjectBudget, TenantBudget},
        error::CostError,
    };

    fn tenant_budget() -> TenantBudget {
        TenantBudget::new("t1", 10.0, 0.8, BudgetPeriod::Daily, 0)
    }

    #[test]
    fn hard_cap_stops_a_run() {
        let mut b = tenant_budget();
        b.record_spend(9.0).ok(); // may trigger soft cap warning
        let err = b.record_spend(2.0).unwrap_err();
        assert!(matches!(err, CostError::HardCapExceeded { .. }));
    }

    #[test]
    fn soft_cap_warns_and_continues() {
        let mut b = tenant_budget();
        // Spend exactly 80% - should trigger SoftCapWarning
        let res = b.record_spend(8.0);
        assert!(matches!(res, Err(CostError::SoftCapWarning { .. })));
        // Budget was still recorded
        assert!((b.spent_usd - 8.0).abs() < 1e-9);
    }

    #[test]
    fn under_soft_cap_is_ok() {
        let mut b = tenant_budget();
        assert!(b.record_spend(7.0).is_ok());
    }

    #[test]
    fn remaining_usd_correct() {
        let mut b = tenant_budget();
        b.record_spend(3.0).ok();
        assert!((b.remaining_usd() - 7.0).abs() < 1e-9);
    }

    #[test]
    fn period_rollover_resets_budget() {
        let mut b = TenantBudget::new("t1", 10.0, 0.8, BudgetPeriod::Daily, 0);
        b.record_spend(5.0).ok();
        b.rollover(86400);
        assert_eq!(b.spent_usd, 0.0);
    }

    #[test]
    fn no_rollover_before_period_ends() {
        let mut b = TenantBudget::new("t1", 10.0, 0.8, BudgetPeriod::Daily, 0);
        b.record_spend(5.0).ok();
        b.rollover(1000);
        assert!((b.spent_usd - 5.0).abs() < 1e-9);
    }

    #[test]
    fn project_budget_hard_cap() {
        let mut p = ProjectBudget::new("proj-1", 50.0, 0.9);
        p.record_spend(40.0).ok();
        let err = p.record_spend(15.0).unwrap_err();
        assert!(matches!(err, CostError::HardCapExceeded { .. }));
    }
}
