# Verification and Abstention

## Step Verification

`StepVerifier::verify` takes a mutable step and a checker function. The checker
receives the claim string and returns `bool`. On return:

- `true` -> step status becomes `Verified`
- `false` -> step status becomes `Refuted`

```rust
StepVerifier::verify(&mut step, |claim| known_facts.contains(claim));
```

## Abstention

`AbstentionPolicy` abstains from a step when aggregated evidence confidence falls
below a configured minimum:

```rust
let policy = AbstentionPolicy::new(0.7); // require 70% confidence
let abstained = policy.apply(&mut step, &[0.4, 0.5]);
// abstained == true, step.status == StepStatus::Abstained
```

Abstention is idempotent: calling `apply` a second time on an already-abstained
step has no additional effect.

## Confidence Aggregation

`ConfidenceAggregator` computes the arithmetic mean of a score slice:

```rust
let agg = ConfidenceAggregator::new(0.6);
agg.is_confident(&[0.7, 0.8]); // true
agg.is_confident(&[0.3, 0.4]); // false
```

An empty score slice produces zero confidence and always triggers abstention.
