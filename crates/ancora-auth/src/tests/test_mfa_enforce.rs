use crate::{MfaChallenge, MfaEnforcer, MfaMethod, MfaStatus};

#[test]
fn mfa_totp_challenge_verified() {
    let mut enforcer = MfaEnforcer::new();
    enforcer.require_for_tenant("tenant-gov");
    let challenge = MfaChallenge::new("ch-1", "alice", MfaMethod::Totp, "123456", 0, 300);
    enforcer.issue_challenge(challenge);
    let ok = enforcer.verify_challenge("ch-1", "123456", 100);
    assert!(ok);
    let ch = enforcer.get_challenge("ch-1").unwrap();
    assert_eq!(ch.status, MfaStatus::Verified);
}

#[test]
fn mfa_wrong_code_fails() {
    let mut enforcer = MfaEnforcer::new();
    let challenge = MfaChallenge::new("ch-2", "bob", MfaMethod::Totp, "654321", 0, 300);
    enforcer.issue_challenge(challenge);
    let ok = enforcer.verify_challenge("ch-2", "000000", 100);
    assert!(!ok);
    let ch = enforcer.get_challenge("ch-2").unwrap();
    assert_eq!(ch.status, MfaStatus::Failed);
}

#[test]
fn mfa_expired_challenge_fails() {
    let mut enforcer = MfaEnforcer::new();
    let challenge = MfaChallenge::new("ch-3", "carol", MfaMethod::HardwareKey, "ok", 0, 50);
    enforcer.issue_challenge(challenge);
    let ok = enforcer.verify_challenge("ch-3", "ok", 100);
    assert!(!ok);
}

#[test]
fn mfa_required_tenant_flag() {
    let mut enforcer = MfaEnforcer::new();
    enforcer.require_for_tenant("gov-tenant");
    assert!(enforcer.is_required("gov-tenant"));
    assert!(!enforcer.is_required("normal-tenant"));
}
