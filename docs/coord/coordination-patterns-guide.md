# Coordination Patterns Guide

ancora-coord provides structured coordination beyond handoff and group chat.

## Blackboard

Shared key-value state visible to all agents. Agents can claim exclusive write
access to a key via `claim_role` to prevent concurrent overwrites.

## Contract-Net

`ContractNet::assign(bids)` selects the agent with the highest score for a task.
Use for task allocation where agents self-report their capability and cost.

## Auction

`Auction::new(task_id)` collects sealed bids and resolves to the winner with
`resolve()`. Equivalent to contract-net but grouped by task.

## Negotiation

`Negotiation::new(max_rounds)` runs a bounded multi-round consensus protocol.
Agents submit `Proposal` values and the result is the integer average.

## Conflict Resolution

`ConflictResolver::resolve(claims, policy)` picks the winner from competing
claims using `HighestPriority`, `FirstCome`, or `Random(seed)` policies.

## Deadlock Detection

`DeadlockDetector` tracks wait-for edges. `has_deadlock()` detects cycles via
DFS. `break_cycle()` removes a single edge to dissolve the deadlock.

## Agent Contracts

`AgentContract::verify_fulfilled(fulfilled_obligations)` checks that all
declared obligations were met before a handoff is accepted.
