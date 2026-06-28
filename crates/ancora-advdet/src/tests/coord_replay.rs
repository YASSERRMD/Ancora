use ancora_coord::{Bid, Blackboard, CoordJournal, ContractNet};

fn record_events(journal: &mut CoordJournal) {
    journal.record(1, "assign", "task-1 -> agent-A");
    journal.record(2, "assign", "task-2 -> agent-B");
    journal.record(3, "complete", "task-1 done");
}

#[test]
fn coord_journal_replay_stable() {
    let mut j1 = CoordJournal::default();
    let mut j2 = CoordJournal::default();
    record_events(&mut j1);
    record_events(&mut j2);

    let r1 = j1.replay();
    let r2 = j2.replay();
    assert_eq!(r1.len(), r2.len());
    for (a, b) in r1.iter().zip(r2.iter()) {
        assert_eq!(a.0, b.0);
        assert_eq!(a.1, b.1);
    }
}

#[test]
fn contract_net_winner_stable() {
    let bids = vec![
        Bid { agent_id: "a1".into(), task_id: "t1".into(), score: 0.7 },
        Bid { agent_id: "a2".into(), task_id: "t1".into(), score: 0.9 },
        Bid { agent_id: "a3".into(), task_id: "t1".into(), score: 0.5 },
    ];
    let w1 = ContractNet::assign(&bids).map(|b| b.agent_id.clone());
    let w2 = ContractNet::assign(&bids).map(|b| b.agent_id.clone());
    assert_eq!(w1, w2);
    assert_eq!(w1.unwrap(), "a2");
}

#[test]
fn blackboard_write_read_stable() {
    let mut b1 = Blackboard::default();
    let mut b2 = Blackboard::default();

    b1.claim_role("planner", "task_list");
    b1.write("planner", "task_list", "step-1,step-2").unwrap();

    b2.claim_role("planner", "task_list");
    b2.write("planner", "task_list", "step-1,step-2").unwrap();

    let v1 = b1.read("task_list");
    let v2 = b2.read("task_list");
    assert_eq!(v1, v2);
}
