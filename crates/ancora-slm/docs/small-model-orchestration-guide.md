# Small-Model Orchestration Guide

This guide covers the orchestration patterns available in `ancora-slm` for running small language models (SLMs) effectively in production agent pipelines.

## Why Small Models First?

Small models (typically < 10B parameters, running locally) offer:

- **Lower latency**: no network round-trip, response in milliseconds on CPU.
- **Lower cost**: no per-token API fees.
- **Privacy**: data never leaves the device.
- **Offline operation**: works without internet connectivity.

The trade-off is reliability: small models make more mistakes, produce malformed output, and struggle with complex multi-step tasks. The patterns in this crate compensate for those weaknesses.

## Core Patterns

### 1. SLM-Friendly Prompt Formatting

Small models are sensitive to prompt format. The wrong format can cut accuracy by 20-40%.

```rust
use ancora_slm::prompt::{format_prompt, PromptOptions, PromptStyle};
use ancora_slm::model::Message;

let messages = vec![
    Message::system("You are a helpful assistant. Extract named entities."),
    Message::user("Alice works at Anthropic."),
];

let opts = PromptOptions {
    style: PromptStyle::Alpaca,   // match the model's training format
    request_json: true,            // append JSON-only instruction
    max_chars: Some(2048),         // keep within context window
};
let prompt = format_prompt(&messages, &opts);
```

**Rule of thumb**: use `Alpaca` for Llama/Mistral-family models, `ChatML` for Phi and Qwen, `Llama2Chat` for Llama 2 specifically.

### 2. Tool-Call Repair

Small models frequently mis-format tool calls. The `repair` module recovers from:
- Prose wrapping the JSON
- Trailing commas
- Single-quoted strings
- Wrong field names (`function_name` instead of `name`)

```rust
use ancora_slm::repair::repair_tool_call;

let raw = r#"Let me call: {"function_name": "search", "args": {"q": "rust"},}"#;
let call = repair_tool_call(raw).expect("repaired");
println!("{} {:?}", call.name, call.arguments);
```

### 3. Constrained Decoding

When the model doesn't support native JSON mode, use the constrained pipeline:

```rust
use ancora_slm::constrained::{run_constrained, ConstrainedConfig};

let config = ConstrainedConfig { max_retries: 3, add_json_fence_instruction: true };
let result = run_constrained("Extract entities", &config, |prompt| model_call(prompt));
```

The pipeline retries with increasingly explicit instructions on failure.

### 4. Schema-Guided Generation

Embed the expected output schema directly in the prompt:

```rust
use ancora_slm::schema::{augment_prompt_with_schema, Schema};

let schema = Schema::Object {
    required: vec!["entities".into()],
    properties: vec![("entities".into(), Schema::Array { item_schema: Box::new(Schema::String) })],
};
let prompt = augment_prompt_with_schema("Extract entities.", &schema);
```

### 5. Step Decomposition

Break complex tasks into steps the SLM can handle one at a time:

```rust
use ancora_slm::decompose::{DecompositionPlan, execute_plan, Step, OutputFormat};

let plan = DecompositionPlan::new("Translate and summarise", vec![
    Step { id: "translate".into(), description: "Translate to English".into(),
           output_format: OutputFormat::Text, optional: false },
    Step { id: "summarise".into(), description: "Summarise in one sentence".into(),
           output_format: OutputFormat::Text, optional: false },
]);
let results = execute_plan(&plan, |prompt| model_call(prompt));
```

### 6. Verifier-Heavy Patterns

Always verify SLM output before acting on it:

```rust
use ancora_slm::verifier::{run_verifiers, ValidJsonVerifier, RequiredKeysVerifier, Verifier};

let verifiers: Vec<Box<dyn Verifier>> = vec![
    Box::new(ValidJsonVerifier),
    Box::new(RequiredKeysVerifier { keys: vec!["entities".into()] }),
];
let report = run_verifiers(&raw_output, &verifiers);
if !report.passed() { /* retry or escalate */ }
```

### 7. Escalation

When the SLM exhausts its retry budget, escalate to a larger model:

```rust
use ancora_slm::escalate::{run_with_escalation, EscalationPolicy};
use ancora_slm::model::ModelTier;

let policy = EscalationPolicy { max_slm_attempts: 2, escalation_tier: ModelTier::Large };
let result = run_with_escalation("task", &policy, &verifiers, slm_fn, llm_fn, "phi-3", "llama-70b");
```

### 8. Few-Shot Injection

Pre-seed the prompt with high-quality examples:

```rust
use ancora_slm::fewshot::{FewShotLibrary, FewShotExample, inject_few_shots};

let mut lib = FewShotLibrary::new();
lib.add(FewShotExample::new("ner", "Input: 'Alice at Acme'", r#"{"entities":["Alice"]}"#, 1.0));
let prompt = inject_few_shots("Input: 'Bob at StartupX'", &lib, "ner", 2);
```

## Deterministic Replay

All model functions in this crate are plain `Fn(&str) -> String`, making them trivially replaceable with replay stubs:

```rust
use ancora_slm::replay::make_replay_fn;

let replay = make_replay_fn(vec![
    ("prompt-a".into(), r#"{"answer": 1}"#.into()),
], "{}");
// Tests always get the same output for the same prompt.
```

## Configuration Tips

| Model family | Recommended style | JSON mode | Tool calls |
|---|---|---|---|
| Llama 2 | `Llama2Chat` | No (use constrained) | No (use repair) |
| Llama 3 / Mistral 7B | `ChatML` | Sometimes | Sometimes |
| Phi-3 mini | `ChatML` | No | No |
| Qwen 2.5 | `ChatML` | Yes | Yes |
| Gemma 2B | `Plain` | No | No |
