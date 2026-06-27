# Error Handling (Rust)

## Error type hierarchy

Ancora errors implement `std::error::Error` and are returned as `anyhow::Error`
by default. Downcast to concrete types with `.is::<T>()` or `.downcast_ref::<T>()`.

| Type | When raised |
|------|------------|
| `AncorError` | Base trait for all Ancora errors |
| `PolicyViolationError` | A policy rule blocked the run |
| `RunFailedError` | Run terminated with an error event |
| `JournalError` | Journal read/write failure |
| `NativeError` | Internal native library failure |

## Pattern matching on error kind

```rust
use ancora_core::{PolicyViolationError, RunFailedError};

match rt.run(&spec, prompt).await {
    Ok(run) => { /* consume events */ }
    Err(e) if e.is::<PolicyViolationError>() => {
        eprintln!("Policy blocked: {}", e);
    }
    Err(e) if e.is::<RunFailedError>() => {
        eprintln!("Run failed: {}", e);
    }
    Err(e) => return Err(e),
}
```

## Retry on transient errors

```rust
use ancora_core::RunFailedError;

async fn run_with_retry(
    rt: &Runtime,
    spec: &AgentSpec,
    prompt: &str,
    max: usize,
) -> anyhow::Result<String> {
    for attempt in 0..max {
        match rt.run(spec, prompt).await {
            Ok(mut run) => {
                while let Some(ev) = run.next().await? {
                    if let RunEvent::Completed { output } = ev {
                        return Ok(output);
                    }
                }
            }
            Err(e) => {
                if attempt + 1 == max { return Err(e); }
                let delay = 2u64.pow(attempt as u32);
                tokio::time::sleep(Duration::from_secs(delay)).await;
            }
        }
    }
    unreachable!()
}
```

## Collecting errors from parallel runs

```rust
use tokio::task::JoinSet;

let mut set = JoinSet::new();
for prompt in prompts {
    let rt = rt.clone();
    let spec = spec.clone();
    set.spawn(async move { run_to_output(&rt, &spec, &prompt).await });
}

let mut results = vec![];
let mut errors = vec![];
while let Some(res) = set.join_next().await {
    match res? {
        Ok(output) => results.push(output),
        Err(e) => errors.push(e),
    }
}
eprintln!("{} errors out of {} runs", errors.len(), results.len() + errors.len());
```

## See also

- [Troubleshooting](troubleshooting.md)
- [Concurrency](concurrency.md)
