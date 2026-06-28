# Datasets and Graders Guide

## Overview

`ancora-evals` provides an offline-capable evaluation platform for the Ancora agent framework.
It supports versioned datasets and multiple scoring strategies via a unified `Grader` trait.

## Datasets

A `Dataset` is a versioned collection of `Example` records. Each example has:
- `id` - a unique identifier
- `input` - the prompt or query passed to the agent
- `expected` - the reference answer or ground truth
- `metadata` - arbitrary key-value pairs for filtering or reporting

### Loading from CSV

```rust
let csv = "q1,What is 2+2?,4\nq2,Capital of France?,Paris";
let dataset = Dataset::from_csv("math-bench", "1.0.0", csv).unwrap();
```

### Importing from Traces

Trace lines use the format `TRACE|id|input|expected`. Use your own import logic
to convert agent execution logs into evaluation examples.

## Graders

All graders implement the `Grader` trait:

```rust
pub trait Grader {
    fn grade(&self, candidate: &str, expected: &str) -> Score;
    fn name(&self) -> &str;
}
```

### Built-in Graders

| Grader | Module | Description |
|---|---|---|
| `ExactMatchGrader` | `exact_match` | 1.0 on exact string match, 0.0 otherwise |
| `SemanticGrader` | `semantic` | Jaccard word-overlap similarity |
| `LlmJudgeGrader` | `llm_judge` | Rubric-based scoring via a judge function |
| `TrajectoryGrader` | `trajectory` | Ordered tool-call sequence comparison |
| `SchemaGrader` | `schema_grader` | Structural validation rules |
| `OfflineJudge` | `offline` | Heuristic local judge - no network required |

## Running an Evaluation

```rust
use ancora_evals::exact_match::ExactMatchGrader;
use ancora_evals::grader::Grader;

let grader = ExactMatchGrader::new().trimmed().case_insensitive();
let score = grader.grade("Paris", "paris");
println!("Score: {}", score.value); // 1.0
```
