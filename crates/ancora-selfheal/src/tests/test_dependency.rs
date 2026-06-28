use crate::dependency::{DependencyHealth, DepStatus};

#[test]
fn all_healthy_initial() {
    let health = DependencyHealth::new();
    assert!(health.is_all_healthy());
}

#[test]
fn one_degraded_not_all_healthy() {
    let mut h = DependencyHealth::new();
    h.report("journal", DepStatus::Degraded { reason: "slow".into() });
    assert!(!h.is_all_healthy());
    assert_eq!(h.degraded_count(), 1);
}

#[test]
fn down_dep_increments_down_count() {
    let mut h = DependencyHealth::new();
    h.report("db", DepStatus::Down { reason: "timeout".into() });
    assert_eq!(h.down_count(), 1);
}

#[test]
fn get_returns_correct_status() {
    let mut h = DependencyHealth::new();
    h.report("cache", DepStatus::Healthy);
    assert_eq!(h.get("cache"), Some(&DepStatus::Healthy));
}
