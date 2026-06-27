# Concurrency (Rust)

## Parallel runs with `JoinSet`

`Runtime` is `Clone` + `Send` + `Sync`. Spawn one task per prompt:

```rust
use ancora_core::{Runtime, AgentSpec, RunEvent};
use tokio::task::JoinSet;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let rt = Runtime::new()?;
    let spec = AgentSpec::builder()
        .model("llama3")
        .instructions("Summarise briefly.")
        .build();

    let prompts = vec!["Text A", "Text B", "Text C"];
    let mut set = JoinSet::new();

    for prompt in prompts {
        let rt = rt.clone();
        let spec = spec.clone();
        set.spawn(async move {
            let mut run = rt.run(&spec, prompt).await?;
            while let Some(ev) = run.next().await? {
                if let RunEvent::Completed { output } = ev {
                    return Ok::<_, anyhow::Error>(output);
                }
            }
            Ok(String::new())
        });
    }

    while let Some(res) = set.join_next().await {
        println!("{}", res??);
    }
    Ok(())
}
```

## Rate-limiting with a `Semaphore`

```rust
use std::sync::Arc;
use tokio::sync::Semaphore;

let sem = Arc::new(Semaphore::new(4));

for prompt in prompts {
    let permit = sem.clone().acquire_owned().await?;
    let rt = rt.clone();
    let spec = spec.clone();
    tokio::spawn(async move {
        let _permit = permit; // released on drop
        let mut run = rt.run(&spec, prompt).await?;
        while let Some(ev) = run.next().await? {
            if let RunEvent::Completed { output } = ev {
                println!("{}", output);
            }
        }
        Ok::<_, anyhow::Error>(())
    });
}
```

## `futures::stream::FuturesUnordered`

For ordered result collection without waiting for the slowest task:

```rust
use futures::{stream::FuturesUnordered, StreamExt};

let mut futures = FuturesUnordered::new();

for prompt in prompts {
    let rt = rt.clone();
    let spec = spec.clone();
    futures.push(async move {
        let mut run = rt.run(&spec, prompt).await?;
        while let Some(ev) = run.next().await? {
            if let RunEvent::Completed { output } = ev { return Ok(output); }
        }
        Ok::<_, anyhow::Error>(String::new())
    });
}

while let Some(result) = futures.next().await {
    println!("{}", result?);
}
```

## See also

- [Streaming](streaming.md)
- [Verifier pattern](verifier.md)
