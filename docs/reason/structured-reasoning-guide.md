# Structured Reasoning Guide

ancora-reason provides composable primitives for verifiable multi-step reasoning.

## Core Concepts

| Primitive | Purpose |
|---|---|
| `StepDecomposer` | Break a goal into ordered reasoning steps |
| `StepVerifier` | Check each step against a checker function |
| `FactChecker` | Ground a claim against a tool lookup |
| `ContradictionDetector` | Identify conflicting claims across steps |
| `EvidenceStore` | Track supporting sources per claim |
| `ConfidenceAggregator` | Compute mean confidence from per-step scores |
| `AbstentionPolicy` | Abstain when evidence confidence is too low |
| `CitationStore` | Attach source citations to claims |
| `ReasoningJournal` | Deterministic event trace for replay |

## Reasoning Chain Example

```rust
use ancora_reason::{StepDecomposer, StepVerifier, ContradictionDetector};

let claims = vec!["A is true".into(), "NOT: A is true".into()];
let mut steps = StepDecomposer::decompose(claims);

// Verify step 0 against an oracle function
StepVerifier::verify(&mut steps[0], |claim| claim == "A is true");

// Detect contradictions
let contradictions = ContradictionDetector::detect(&steps);
assert_eq!(contradictions.len(), 1);
```

## Contradiction Detection

Claims that contradict each other use the `NOT: ` prefix convention:

- `"X is true"` and `"NOT: X is true"` are contradictions
- `ContradictionDetector::detect` returns all contradicting pairs as `(usize, usize)` indices
