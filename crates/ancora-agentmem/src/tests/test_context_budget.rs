use crate::context_budget::ContextBudget;

#[test]
fn remaining_decreases_on_add() {
    let mut b = ContextBudget::new(1000, 100);
    b.add_message(200);
    assert_eq!(b.remaining(), 700);
}

#[test]
fn add_message_fails_when_over_budget() {
    let mut b = ContextBudget::new(1000, 0);
    assert!(!b.add_message(1001));
}

#[test]
fn reset_clears_used() {
    let mut b = ContextBudget::new(1000, 50);
    b.add_message(400);
    b.reset_to_system();
    assert_eq!(b.used(), 50);
}

#[test]
fn utilization_pct_correct() {
    let mut b = ContextBudget::new(1000, 0);
    b.add_message(500);
    assert!((b.utilization_pct() - 50.0).abs() < 0.001);
}
