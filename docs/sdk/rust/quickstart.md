# Quickstart (Rust)

## Minimal single-agent example

```rust
use ancora_core::{Runtime, AgentSpec, RunEvent};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let rt = Runtime::new()?;
    let spec = AgentSpec::builder()
        .model("llama3")
        .instructions("You are a concise assistant.")
        .build();

    let mut run = rt.run(&spec, "What is the capital of France?").await?;

    while let Some(event) = run.next().await? {
        match event {
            RunEvent::Token { token } => print!("{}", token),
            RunEvent::Completed { output } => {
                println!();
                eprintln!("Final: {}", output);
            }
            _ => {}
        }
    }

    Ok(())
}
```

## What each line does

| Line | Purpose |
|------|---------|
| `Runtime::new()` | Load the native Ancora library and connect to the inference backend |
| `AgentSpec::builder()` | Build the agent configuration |
| `rt.run(&spec, prompt)` | Start a run; returns a `Run` handle |
| `run.next().await` | Stream the next `RunEvent` |
| `RunEvent::Token` | Streamed text fragment |
| `RunEvent::Completed` | Run finished; `output` is the full final text |

## Using a hosted model

Set the endpoint via environment variable before running:

```bash
export ANTHROPIC_API_KEY=sk-...
```

```rust
let spec = AgentSpec::builder()
    .model("claude-sonnet-4-6")
    .instructions("You are a concise assistant.")
    .build();
```

Ancora selects the provider from the model name prefix.

## See also

- [Streaming](streaming.md)
- [Tools](tools.md)
