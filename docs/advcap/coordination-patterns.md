# Coordination Patterns

ancora-coord provides shared-state, auction, negotiation, conflict resolution,
and deadlock detection primitives for multi-agent systems.

## Blackboard

Shared state with role-based write access:

```rust
use ancora_coord::Blackboard;

let mut board = Blackboard::default();
board.claim_role("planner", "task_list");
board.write("planner", "task_list", "search, summarize").unwrap();
board.write("executor", "task_list", "other").unwrap_err(); // PermissionDenied
```

## Contract-net Bidding

```rust
use ancora_coord::{ContractNet, Bid};

let bids = vec![
    Bid { agent_id: "a1".into(), task_id: "t1".into(), score: 0.8 },
    Bid { agent_id: "a2".into(), task_id: "t1".into(), score: 0.9 },
];
let winner = ContractNet::assign(&bids); // returns agent with highest score
```

## Negotiation

```rust
use ancora_coord::Negotiation;

let mut neg = Negotiation::new(3); // max 3 rounds
neg.submit(Proposal { agent_id: "a1".into(), value: 100 })?;
neg.submit(Proposal { agent_id: "a2".into(), value: 100 })?;
assert!(neg.converged()); // all proposals equal
```

## Deadlock Detection

```rust
use ancora_coord::DeadlockDetector;

let mut detector = DeadlockDetector::new();
detector.add_wait("a1", "a2");
detector.add_wait("a2", "a1");
assert!(detector.has_deadlock());
detector.break_cycle(); // removes one edge
```

## Agent Contracts

```rust
use ancora_coord::AgentContract;

let contract = AgentContract {
    contract_id: "c1".into(), from_agent: "a1".into(), to_agent: "a2".into(),
    obligations: vec!["summarize".into(), "respond".into()],
};
contract.verify_fulfilled(&["summarize", "respond"])?; // Ok
```
