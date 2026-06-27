# Testing (Rust)

## Offline `tokio::test` patterns

Use `MemoryStore` to avoid any file I/O in tests:

```rust
use ancora_core::{Runtime, RuntimeOptions, MemoryStore, StoringTransport, AgentSpec, RunEvent};

#[tokio::test]
async fn test_agent_replies() -> anyhow::Result<()> {
    let store = MemoryStore::new();
    let rt = Runtime::with_options(RuntimeOptions {
        transport: Some(Box::new(StoringTransport::new(store))),
        model_url: Some("http://127.0.0.1:11434".into()),
        ..Default::default()
    })?;

    let spec = AgentSpec::builder()
        .model("llama3")
        .instructions("You are a helpful assistant.")
        .build();

    let mut output = String::new();
    let mut run = rt.run(&spec, "Say hi.").await?;

    while let Some(ev) = run.next().await? {
        if let RunEvent::Completed { output: o } = ev {
            output = o;
        }
    }

    assert!(!output.is_empty(), "agent should return a non-empty reply");
    Ok(())
}
```

## Skipping tests when no inference backend is available

```rust
#[tokio::test]
async fn test_requires_ollama() -> anyhow::Result<()> {
    if std::env::var("CI").is_ok() {
        eprintln!("Skipping: no Ollama in CI");
        return Ok(());
    }
    // test body ...
    Ok(())
}
```

Or use `rstest` with `#[ignore]`:

```rust
#[tokio::test]
#[ignore = "requires live Ollama"]
async fn test_live_model() -> anyhow::Result<()> {
    // ...
    Ok(())
}
```

Run ignored tests explicitly: `cargo test -- --ignored`.

## Checking journal replay

```rust
#[tokio::test]
async fn test_replay_is_deterministic() -> anyhow::Result<()> {
    let store = MemoryStore::new();
    // ... run once, capture output
    // ... resume with same run_id
    // ... assert replayed output equals first output
    Ok(())
}
```

## See also

- [Durability](durability.md)
- [Troubleshooting](troubleshooting.md)
