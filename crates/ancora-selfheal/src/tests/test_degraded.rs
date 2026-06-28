use crate::degraded_mode::{DegradedController, SystemMode};

#[test]
fn normal_mode_accepts_runs() {
    let c = DegradedController::new();
    assert!(c.is_accepting_runs());
}

#[test]
fn degraded_mode_still_accepts_runs() {
    let mut c = DegradedController::new();
    c.enter_degraded(vec!["streaming".into()]);
    assert!(c.is_accepting_runs());
    assert!(matches!(c.mode, SystemMode::Degraded { .. }));
}

#[test]
fn emergency_mode_rejects_runs() {
    let mut c = DegradedController::new();
    c.enter_emergency();
    assert!(!c.is_accepting_runs());
}

#[test]
fn recover_returns_to_normal() {
    let mut c = DegradedController::new();
    c.enter_emergency();
    c.recover();
    assert_eq!(c.mode, SystemMode::Normal);
}
