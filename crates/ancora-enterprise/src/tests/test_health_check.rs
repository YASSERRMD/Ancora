use crate::checkpoint::{CheckStatus, HealthCheck};

#[test]
fn pass_check() {
    let c = HealthCheck::new("c1", "HSM", "hsm", CheckStatus::Pass, "All good", 1);
    assert!(c.is_healthy());
    assert!(!c.is_failing());
    assert!(!c.is_warning());
}

#[test]
fn warn_check() {
    let c = HealthCheck::new("c1", "Incidents", "ops", CheckStatus::Warn, "2 open", 1);
    assert!(!c.is_healthy());
    assert!(!c.is_failing());
    assert!(c.is_warning());
}

#[test]
fn fail_check() {
    let c = HealthCheck::new(
        "c1",
        "License",
        "licensing",
        CheckStatus::Fail,
        "Expired",
        1,
    );
    assert!(!c.is_healthy());
    assert!(c.is_failing());
    assert!(!c.is_warning());
}
