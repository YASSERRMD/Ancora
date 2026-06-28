use crate::cost_view::{CostBreakdown, StepCost};

fn make_step(index: usize, model: &str, tin: u32, tout: u32, cost: f64) -> StepCost {
    StepCost {
        step_index: index,
        model: model.into(),
        tokens_in: tin,
        tokens_out: tout,
        cost_usd: cost,
    }
}

#[test]
fn test_cost_view_renders() {
    let b = CostBreakdown::new(
        "r1",
        vec![
            make_step(0, "gpt-4", 200, 100, 0.006),
            make_step(1, "gpt-4", 100, 50, 0.003),
        ],
    );
    assert_eq!(b.steps.len(), 2);
    assert!((b.total_cost_usd() - 0.009).abs() < 1e-9);
}

#[test]
fn test_cost_tokens_sum() {
    let b = CostBreakdown::new(
        "r1",
        vec![
            make_step(0, "gpt-4", 100, 50, 0.003),
            make_step(1, "gpt-3.5", 80, 40, 0.001),
        ],
    );
    assert_eq!(b.total_tokens_in(), 180);
    assert_eq!(b.total_tokens_out(), 90);
}

#[test]
fn test_cost_by_model() {
    let b = CostBreakdown::new(
        "r1",
        vec![
            make_step(0, "gpt-4", 100, 50, 0.003),
            make_step(1, "gpt-4", 100, 50, 0.003),
            make_step(2, "gpt-3.5", 80, 40, 0.001),
        ],
    );
    let by_model = b.cost_by_model();
    assert!((by_model["gpt-4"] - 0.006).abs() < 1e-9);
    assert!((by_model["gpt-3.5"] - 0.001).abs() < 1e-9);
}

#[test]
fn test_average_cost_per_step() {
    let b = CostBreakdown::new(
        "r1",
        vec![
            make_step(0, "m", 10, 5, 0.002),
            make_step(1, "m", 10, 5, 0.004),
        ],
    );
    assert!((b.average_cost_per_step() - 0.003).abs() < 1e-9);
}
