use crate::contract_net::{Bid, ContractNet};

#[test]
fn contract_net_assigns_to_best_bid() {
    let bids = vec![
        Bid {
            agent_id: "a".into(),
            task_id: "t1".into(),
            score: 0.5,
        },
        Bid {
            agent_id: "b".into(),
            task_id: "t1".into(),
            score: 0.9,
        },
        Bid {
            agent_id: "c".into(),
            task_id: "t1".into(),
            score: 0.3,
        },
    ];
    let winner = ContractNet::assign(&bids).unwrap();
    assert_eq!(winner.agent_id, "b");
}

#[test]
fn contract_net_empty_bids_returns_none() {
    assert!(ContractNet::assign(&[]).is_none());
}
