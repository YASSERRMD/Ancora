# Customizing Recipes

Every recipe accepts a `ParamSet` to override default values.

## Using ParamSet

```rust
use ancora_recipe::params::{ParamSet, apply_override};
use ancora_recipe::rag_citations;

let mut ps = ParamSet::new();
ps.set("corpus", "my-knowledge-base");
ps.set("top_k", "10");

let recipe = rag_citations::build(&ps);
```

## Applying CLI overrides

Use `apply_override` to parse `key=value` strings from command-line arguments:

```rust
let mut ps = ParamSet::new();
apply_override(&mut ps, "top_k=8").unwrap();
apply_override(&mut ps, "corpus=internal-docs").unwrap();
```

## Layering defaults with user overrides

```rust
let defaults = ParamSet::from_pairs([("top_k", "5"), ("corpus", "default")]);
let mut user_ps = ParamSet::from_pairs([("top_k", "10")]);
user_ps.merge(&defaults); // defaults fill in missing keys only
```

## Recipe-specific parameters

| Recipe | Parameter | Default | Effect |
|--------|-----------|---------|--------|
| `rag-citations` | `corpus` | `documents` | Corpus name in retrieval step |
| `rag-citations` | `top_k` | `5` | Number of passages to retrieve |
| `research-report` | `topic` | `general topic` | Report subject |
| `research-report` | `sections` | `3` | Number of outline sections |
| `code-review` | `language` | `any` | Programming language hint |
| `code-review` | `strict` | `false` | Enable strict lint rules |
| `customer-support` | `product` | `product` | Product name for context |
| `customer-support` | `escalation_threshold` | `3` | Turns before escalation |
| `multi-agent-debate` | `rounds` | `2` | Number of debate rounds |
| `multi-agent-debate` | `agents` | `2` | Number of debating agents |
| `document-processing` | `doc_type` | `generic` | Document type label |
| `document-processing` | `chunking` | `paragraph` | Chunking strategy |
