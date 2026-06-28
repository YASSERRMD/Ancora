# Choosing a Coordination Pattern

| Pattern | Best for |
|---|---|
| Blackboard | Shared context that many agents read and few write |
| Contract-Net | Dynamic task allocation based on self-reported capability |
| Auction | Single winner from competing bids with clear scoring |
| Negotiation | Convergence to consensus across multiple rounds |
| Conflict | Deterministic resolution when two agents claim the same resource |
| Deadlock | Cycle breaking in wait-for graphs |
| Contract | Formal obligation verification before handoff |

## Combining Patterns

Start with a Blackboard for shared state, run an Auction or Contract-Net to
assign subtasks, use Negotiation for parameters that require consensus, and
verify Agent Contracts before accepting handoffs.
