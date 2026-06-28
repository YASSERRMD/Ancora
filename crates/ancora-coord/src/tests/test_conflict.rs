use crate::conflict::{Claim, ConflictPolicy, ConflictResolver};

fn claim(agent_id: &str, priority: u32, arrived_at: u64) -> Claim {
    Claim { agent_id: agent_id.to_string(), priority, arrived_at }
}

#[test]
fn conflict_resolved_by_highest_priority() {
    let claims = vec![claim("a", 5, 1), claim("b", 9, 2), claim("c", 3, 3)];
    let winner = ConflictResolver::resolve(&claims, &ConflictPolicy::HighestPriority).unwrap();
    assert_eq!(winner.agent_id, "b");
}

#[test]
fn conflict_resolved_by_first_come() {
    let claims = vec![claim("a", 5, 3), claim("b", 9, 1), claim("c", 3, 2)];
    let winner = ConflictResolver::resolve(&claims, &ConflictPolicy::FirstCome).unwrap();
    assert_eq!(winner.agent_id, "b");
}

#[test]
fn empty_claims_returns_none() {
    assert!(ConflictResolver::resolve(&[], &ConflictPolicy::HighestPriority).is_none());
}
