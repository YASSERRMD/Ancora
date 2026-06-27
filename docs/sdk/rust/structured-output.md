# Structured Output (Rust)

## Parse run output with `serde`

```rust
use ancora_core::{AgentSpec, Runtime, RunEvent};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Sentiment {
    label: String,
    score: f32,
    reasoning: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let rt = Runtime::new()?;
    let spec = AgentSpec::builder()
        .model("llama3")
        .instructions(
            "Return ONLY valid JSON with keys: label (positive/negative/neutral), \
             score (0.0-1.0), reasoning (one sentence).",
        )
        .build();

    let mut run = rt.run(&spec, "Rust is the best systems language.").await?;
    let mut output = String::new();

    while let Some(ev) = run.next().await? {
        match ev {
            RunEvent::Token { token } => output.push_str(&token),
            RunEvent::Completed { .. } => break,
            _ => {}
        }
    }

    let sentiment: Sentiment = serde_json::from_str(&output)?;
    println!("{:?}", sentiment);
    Ok(())
}
```

## Using `serde_json::Value` for dynamic output

```rust
let value: serde_json::Value = serde_json::from_str(&output)?;
let label = value["label"].as_str().unwrap_or("unknown");
```

## Schema enforcement via instructions

Prepend a JSON schema description to the `instructions` field.
The model is instructed to conform; Ancora does not validate at the
protocol layer in Rust -- parsing failure is an application concern.

## Extracting from partial stream

Collect tokens into a `String` and parse after `RunEvent::Completed`:

```rust
let mut buf = String::new();
while let Some(ev) = run.next().await? {
    match ev {
        RunEvent::Token { token } => buf.push_str(&token),
        RunEvent::Completed { .. } => break,
        _ => {}
    }
}
let parsed: MyStruct = serde_json::from_str(&buf)?;
```

## See also

- [Streaming](streaming.md)
- [API reference](api-reference.md)
