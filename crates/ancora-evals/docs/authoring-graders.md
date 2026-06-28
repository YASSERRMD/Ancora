# Authoring Graders

## Implementing the `Grader` Trait

Any struct that implements the `Grader` trait can be used in the eval pipeline.

```rust
use ancora_evals::grader::{Grader, Score};

pub struct WordCountGrader {
    pub target_words: usize,
}

impl Grader for WordCountGrader {
    fn grade(&self, candidate: &str, _expected: &str) -> Score {
        let count = candidate.split_whitespace().count();
        let diff = (count as isize - self.target_words as isize).unsigned_abs();
        let value = 1.0_f64 - (diff as f64 / self.target_words as f64).min(1.0);
        Score::new(value.clamp(0.0, 1.0))
    }

    fn name(&self) -> &str {
        "word_count"
    }
}
```

## Registering a Custom Grader

Use `GraderRegistry` to register and retrieve graders by name at runtime:

```rust
use ancora_evals::registry::GraderRegistry;

let mut registry = GraderRegistry::new();
registry.register("word_count", WordCountGrader { target_words: 50 });

let score = registry.grade("word_count", candidate, expected).unwrap();
```

## Offline Grading

The `OfflineJudge` provides scoring without network access, using word-overlap
heuristics. Use it when an LLM API is unavailable or for CI environments:

```rust
use ancora_evals::offline::{OfflineJudge, run_offline_batch};

let judge = OfflineJudge::new().with_overlap_threshold(0.5);
let results = run_offline_batch(
    &judge,
    vec![("id1", "candidate answer", "expected answer")].into_iter(),
);
```

## Rubric-based LLM Judge

Define criteria with weights and a judge function:

```rust
use ancora_evals::llm_judge::{Criterion, LlmJudgeGrader, Rubric};

let rubric = Rubric::new()
    .add_criterion(Criterion::new("accuracy", "Is the answer correct?", 0.7))
    .add_criterion(Criterion::new("conciseness", "Is the answer concise?", 0.3));

let grader = LlmJudgeGrader::offline(rubric);
```

## Best Practices

- Keep graders stateless when possible for determinism.
- Use `Score::with_rationale` to explain the score for debugging.
- Always clamp scores to [0.0, 1.0] before constructing `Score`.
- Prefer `LlmJudgeGrader::offline` in test environments to avoid flakiness.
