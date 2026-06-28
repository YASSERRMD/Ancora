use crate::approval::ApprovalGate;

#[test]
fn unapproved_tool_blocked() {
    let gate = ApprovalGate::default();
    assert!(gate.check("my_tool").is_err());
}

#[test]
fn approved_tool_passes() {
    let mut gate = ApprovalGate::default();
    gate.approve("my_tool");
    assert!(gate.check("my_tool").is_ok());
}

#[test]
fn revoked_tool_blocked_again() {
    let mut gate = ApprovalGate::default();
    gate.approve("my_tool");
    gate.revoke("my_tool");
    assert!(gate.check("my_tool").is_err());
}

#[test]
fn is_approved_reflects_state() {
    let mut gate = ApprovalGate::default();
    assert!(!gate.is_approved("t"));
    gate.approve("t");
    assert!(gate.is_approved("t"));
}
