# Migration Guide (Rust)

## From `async-openai`

`async-openai` gives you raw API access. Ancora wraps the model call in a
run with tool dispatch, journal replay, and policy enforcement.

| `async-openai` | Ancora equivalent |
|---------------|-------------------|
| `Client::new()` | `Runtime::new()` |
| `ChatCompletionRequestBuilder` | `AgentSpec::builder()` |
| `client.chat().create(req)` | `rt.run(&spec, prompt).await?` |
| Collect `delta.content` | `RunEvent::Token { token }` |
| Final message | `RunEvent::Completed { output }` |

### Before (async-openai)

```rust
use async_openai::{Client, types::*};

let client = Client::new();
let req = CreateChatCompletionRequestArgs::default()
    .model("gpt-4o")
    .messages([ChatCompletionRequestUserMessage::from("Hello")])
    .build()?;
let resp = client.chat().create(req).await?;
println!("{}", resp.choices[0].message.content.as_deref().unwrap_or(""));
```

### After (Ancora)

```rust
use ancora_core::{Runtime, AgentSpec, RunEvent};

let rt = Runtime::new()?;
let spec = AgentSpec::builder().model("gpt-4o").instructions("").build();
let mut run = rt.run(&spec, "Hello").await?;
while let Some(ev) = run.next().await? {
    if let RunEvent::Completed { output } = ev { println!("{}", output); }
}
```

## From LLM crate

```rust
// Before
let llm = llm::load_llm(model_path)?;
let session = llm.start_session(Default::default());
session.feed_prompt(params, &model, &prompt, None, |t| { /* ... */ });

// After -- Ancora handles session lifecycle, tokenization, and tool dispatch
let rt = Runtime::new()?;
let mut run = rt.run(&spec, &prompt).await?;
while let Some(ev) = run.next().await? {
    if let RunEvent::Token { token } = ev { print!("{}", token); }
}
```

## From `reqwest` raw HTTP calls

Replace manual JSON construction and HTTP calls with Ancora's typed API.
Tool calling, retry, and streaming are handled automatically.

## See also

- [Quickstart](quickstart.md)
- [Providers](providers.md)
