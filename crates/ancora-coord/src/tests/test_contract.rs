use crate::contract::AgentContract;

#[test]
fn fulfilled_contract_passes() {
    let c = AgentContract::new("c1", "a", "b", vec!["send-result", "notify-done"]);
    assert!(c.verify_fulfilled(&["send-result", "notify-done"]).is_ok());
}

#[test]
fn contract_violation_rejected() {
    let c = AgentContract::new("c1", "a", "b", vec!["send-result", "notify-done"]);
    assert!(c.verify_fulfilled(&["send-result"]).is_err());
}

#[test]
fn empty_obligations_always_fulfilled() {
    let c = AgentContract::new("c2", "a", "b", vec![]);
    assert!(c.verify_fulfilled(&[]).is_ok());
}
