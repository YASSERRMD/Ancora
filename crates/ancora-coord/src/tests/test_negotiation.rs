use crate::negotiation::{Negotiation, Proposal};

#[test]
fn negotiation_converges_in_bounded_rounds() {
    let mut neg = Negotiation::new(5);
    neg.submit(Proposal { agent_id: "a".into(), value: 10 }).unwrap();
    neg.submit(Proposal { agent_id: "b".into(), value: 10 }).unwrap();
    assert!(neg.converged());
}

#[test]
fn negotiation_consensus_is_average() {
    let mut neg = Negotiation::new(5);
    neg.submit(Proposal { agent_id: "a".into(), value: 10 }).unwrap();
    neg.submit(Proposal { agent_id: "b".into(), value: 20 }).unwrap();
    assert_eq!(neg.consensus(), Some(15));
}

#[test]
fn negotiation_max_rounds_enforced() {
    let mut neg = Negotiation::new(1);
    neg.submit(Proposal { agent_id: "a".into(), value: 5 }).unwrap();
    assert!(neg.submit(Proposal { agent_id: "b".into(), value: 5 }).is_err());
}
