use crate::checkpoint::{CheckStatus, EnterpriseCheckpoint, HealthCheck};

fn chk(id: &str, domain: &str, status: CheckStatus) -> HealthCheck {
    HealthCheck::new(id, "N", domain, status, "msg", 1)
}

#[test]
fn empty_checkpoint() {
    let cp = EnterpriseCheckpoint::new(1);
    assert_eq!(cp.count(), 0);
    assert!(cp.all_healthy());
    assert!((cp.pass_rate() - 0.0).abs() < f64::EPSILON);
}

#[test]
fn all_passing() {
    let mut cp = EnterpriseCheckpoint::new(1);
    cp.add(chk("c1", "hsm", CheckStatus::Pass));
    cp.add(chk("c2", "audit", CheckStatus::Pass));
    assert!(cp.all_healthy());
    assert!((cp.pass_rate() - 1.0).abs() < 1e-9);
}

#[test]
fn with_failure() {
    let mut cp = EnterpriseCheckpoint::new(1);
    cp.add(chk("c1", "hsm", CheckStatus::Pass));
    cp.add(chk("c2", "licensing", CheckStatus::Fail));
    assert!(!cp.all_healthy());
    assert_eq!(cp.failing().len(), 1);
    assert!((cp.pass_rate() - 0.5).abs() < 1e-9);
}

#[test]
fn with_warning() {
    let mut cp = EnterpriseCheckpoint::new(1);
    cp.add(chk("c1", "ops", CheckStatus::Warn));
    assert_eq!(cp.warnings().len(), 1);
    assert!(cp.all_healthy());
}

#[test]
fn for_domain() {
    let mut cp = EnterpriseCheckpoint::new(1);
    cp.add(chk("c1", "hsm", CheckStatus::Pass));
    cp.add(chk("c2", "audit", CheckStatus::Pass));
    assert_eq!(cp.for_domain("hsm").len(), 1);
    assert_eq!(cp.for_domain("other").len(), 0);
}
