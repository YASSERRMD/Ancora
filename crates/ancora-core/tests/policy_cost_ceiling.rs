// Policy: cost ceiling -- reject or abort runs that would exceed budget.

struct CostPolicy {
    ceiling_usd: f64,
    accumulated_usd: f64,
}

impl CostPolicy {
    fn new(ceiling_usd: f64) -> Self { Self { ceiling_usd, accumulated_usd: 0.0 } }

    fn record(&mut self, cost: f64) -> Result<(), String> {
        if self.accumulated_usd + cost > self.ceiling_usd {
            Err(format!("cost ceiling exceeded: {:.4} + {:.4} > {:.4}",
                self.accumulated_usd, cost, self.ceiling_usd))
        } else {
            self.accumulated_usd += cost;
            Ok(())
        }
    }

    fn remaining(&self) -> f64 { (self.ceiling_usd - self.accumulated_usd).max(0.0) }
}

#[test]
fn test_cost_within_ceiling_recorded() {
    let mut p = CostPolicy::new(1.0);
    p.record(0.50).unwrap();
    assert!((p.accumulated_usd - 0.50).abs() < 0.0001);
}

#[test]
fn test_cost_exceeds_ceiling_rejected() {
    let mut p = CostPolicy::new(0.10);
    let r = p.record(0.20);
    assert!(r.is_err());
    assert!(r.unwrap_err().contains("exceeded"));
}

#[test]
fn test_accumulated_cost_increments() {
    let mut p = CostPolicy::new(1.0);
    p.record(0.30).unwrap();
    p.record(0.30).unwrap();
    assert!((p.accumulated_usd - 0.60).abs() < 0.0001);
}

#[test]
fn test_remaining_budget_correct() {
    let mut p = CostPolicy::new(1.0);
    p.record(0.25).unwrap();
    assert!((p.remaining() - 0.75).abs() < 0.0001);
}

#[test]
fn test_exact_ceiling_allowed() {
    let mut p = CostPolicy::new(0.50);
    p.record(0.50).unwrap();
    assert!((p.remaining()).abs() < 0.0001);
}

#[test]
fn test_one_cent_over_ceiling_rejected() {
    let mut p = CostPolicy::new(0.50);
    p.record(0.49).unwrap();
    let r = p.record(0.02);
    assert!(r.is_err());
}

#[test]
fn test_zero_ceiling_rejects_any_cost() {
    let mut p = CostPolicy::new(0.0);
    let r = p.record(0.001);
    assert!(r.is_err());
}
