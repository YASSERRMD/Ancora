# Structured Reasoning

ancora-reason provides verifiable multi-step reasoning with claim decomposition,
step verification, fact grounding, contradiction detection, evidence tracking,
confidence-based abstention, and citation output.

## Reasoning Pipeline

```rust
use ancora_reason::{StepDecomposer, StepVerifier, ContradictionDetector, AbstentionPolicy};

let mut steps = StepDecomposer::decompose(vec!["claim-A".into(), "claim-B".into()]);

// Verify each step
for step in steps.iter_mut() {
    StepVerifier::verify(step, |claim| oracle.check(claim));
}

// Detect contradictions
let contradictions = ContradictionDetector::detect(&steps);

// Abstain on low-confidence steps
let policy = AbstentionPolicy::new(0.7);
policy.apply(&mut steps[0], &[0.3, 0.4]); // abstains
```

## Fact Grounding

```rust
use ancora_reason::FactChecker;

let fc = FactChecker::check("water boils at 100C", |claim| {
    db.lookup(claim) // returns Option<String>
});
if fc.grounded { evidence.add(&fc.claim, fc.source); }
```

## Contradiction Convention

Claims that contradict each other use the `NOT: ` prefix:
- `"gravity exists"` and `"NOT: gravity exists"` are a contradiction pair

## Eval

```rust
use ancora_ageval::ReasoningMetric;
let score = ReasoningMetric::score(verified_count, total_steps);
```
