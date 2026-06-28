use crate::blackboard::Blackboard;

#[test]
fn blackboard_updates_visible_to_agents() {
    let mut bb = Blackboard::default();
    bb.write("agent-a", "status", "ready").unwrap();
    assert_eq!(bb.read("status"), Some("ready"));
}

#[test]
fn role_based_access_enforced() {
    let mut bb = Blackboard::default();
    bb.claim_role("agent-a", "result");
    bb.write("agent-a", "result", "42").unwrap();
    assert!(bb.write("agent-b", "result", "99").is_err());
}

#[test]
fn unclaimed_key_writable_by_any() {
    let mut bb = Blackboard::default();
    assert!(bb.write("agent-x", "free-key", "val").is_ok());
}
