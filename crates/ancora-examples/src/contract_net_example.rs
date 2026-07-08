use ancora_coord::{
    AgentContract, Auction, Bid, Blackboard, Claim, ConflictPolicy, ConflictResolver, ContractNet,
    CoordJournal, DeadlockDetector, Negotiation, Proposal,
};

pub fn run_contract_net_example() {
    let mut bb = Blackboard::default();
    bb.write("agent-a", "status", "ready").unwrap();
    assert_eq!(bb.read("status"), Some("ready"));

    let bids = vec![
        Bid {
            agent_id: "a".into(),
            task_id: "t1".into(),
            score: 0.6,
        },
        Bid {
            agent_id: "b".into(),
            task_id: "t1".into(),
            score: 0.9,
        },
    ];
    let winner = ContractNet::assign(&bids).unwrap();
    assert_eq!(winner.agent_id, "b");

    let mut auction = Auction::new("task-2");
    auction.submit(Bid {
        agent_id: "x".into(),
        task_id: "task-2".into(),
        score: 0.7,
    });
    auction.submit(Bid {
        agent_id: "y".into(),
        task_id: "task-2".into(),
        score: 0.5,
    });
    assert_eq!(auction.resolve().unwrap().agent_id, "x");

    let mut neg = Negotiation::new(3);
    neg.submit(Proposal {
        agent_id: "a".into(),
        value: 8,
    })
    .unwrap();
    neg.submit(Proposal {
        agent_id: "b".into(),
        value: 12,
    })
    .unwrap();
    assert_eq!(neg.consensus(), Some(10));

    let claims = vec![
        Claim {
            agent_id: "p".into(),
            priority: 5,
            arrived_at: 1,
        },
        Claim {
            agent_id: "q".into(),
            priority: 9,
            arrived_at: 2,
        },
    ];
    let winner = ConflictResolver::resolve(&claims, &ConflictPolicy::HighestPriority).unwrap();
    assert_eq!(winner.agent_id, "q");

    let mut det = DeadlockDetector::default();
    det.add_wait("m", "n");
    det.add_wait("n", "m");
    assert!(det.has_deadlock());
    det.break_cycle().unwrap();

    let contract = AgentContract::new("c1", "a", "b", vec!["deliver-result"]);
    assert!(contract.verify_fulfilled(&["deliver-result"]).is_ok());

    let mut journal = CoordJournal::default();
    journal.record(1, "assign", "b won task-1");
    journal.record(2, "conflict", "q won by priority");
    let replayed = journal.replay();
    assert_eq!(replayed.len(), 2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contract_net_example_runs() {
        run_contract_net_example();
    }
}
