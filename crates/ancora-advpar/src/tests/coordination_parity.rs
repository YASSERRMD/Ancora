use ancora_ageval::CoordinationMetric;
use ancora_coord::{Bid, Blackboard, ContractNet, CoordJournal};

const EPS: f64 = 1e-9;

#[test]
fn coordination_parity_score_canonical() {
    assert!((CoordinationMetric::score(3, 3) - 1.0).abs() < EPS);
    assert!((CoordinationMetric::score(4, 3) - 0.75).abs() < EPS);
    assert!((CoordinationMetric::score(5, 2) - 0.4).abs() < EPS);
}

#[test]
fn coordination_parity_contract_net_winner() {
    let bids = vec![
        Bid {
            agent_id: "a".into(),
            task_id: "t".into(),
            score: 0.6,
        },
        Bid {
            agent_id: "b".into(),
            task_id: "t".into(),
            score: 0.9,
        },
    ];
    assert_eq!(
        ContractNet::assign(&bids).map(|b| b.agent_id.as_str()),
        Some("b")
    );
}

#[test]
fn coordination_parity_blackboard_write_read() {
    let mut board = Blackboard::default();
    board.claim_role("agent1", "goal");
    board.write("agent1", "goal", "solve-parity").unwrap();
    assert_eq!(board.read("goal"), Some("solve-parity"));
}

#[test]
fn coordination_parity_journal_replay() {
    let mut j = CoordJournal::default();
    j.record(1, "bid", "task-1");
    j.record(2, "assign", "task-1 -> agent-B");
    let r = j.replay();
    assert_eq!(r.len(), 2);
    assert_eq!(r[0].0, "bid");
    assert_eq!(r[1].0, "assign");
}

#[test]
fn coordination_parity_empty_bids() {
    assert!(ContractNet::assign(&[]).is_none());
}
