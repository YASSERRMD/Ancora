# Streaming (Rust)

## `Run` event iterator

`rt.run()` returns a `Run` handle. Call `.next().await` to receive events one
at a time:

```rust
use ancora_core::{AgentSpec, Runtime, RunEvent};

let mut run = rt.run(&spec, prompt).await?;
while let Some(ev) = run.next().await? {
    match ev {
        RunEvent::Started { run_id } => eprintln!("Started: {}", run_id),
        RunEvent::Token { token } => print!("{}", token),
        RunEvent::ToolCall { name, input } => {
            eprintln!("Calling tool: {} with {:?}", name, input);
        }
        RunEvent::Completed { output } => {
            println!();
            eprintln!("Done. Output: {}", output);
        }
        RunEvent::Suspended { reason } => eprintln!("Suspended: {}", reason),
        RunEvent::Resumed { .. } => {}
    }
}
```

## All `RunEvent` variants

| Variant | Fields | When emitted |
|---------|--------|--------------|
| `Started` | `run_id: String` | Run begins |
| `Token` | `token: String` | Each streamed text fragment |
| `ToolCall` | `name, input` | Before tool handler fires |
| `Completed` | `output: String` | Run ends successfully |
| `Suspended` | `reason: String` | Run paused for human input |
| `Resumed` | (none) | After `run.resume()` called |

## Converting to a stream

Use `futures::stream::unfold` to wrap the event loop:

```rust
use futures::stream;

let stream = stream::unfold(run, |mut run| async {
    match run.next().await {
        Ok(Some(ev)) => Some((ev, run)),
        _ => None,
    }
});
```

## Cancellation

Drop the `Run` handle to cancel the stream. The underlying inference call
is aborted.

```rust
{
    let mut run = rt.run(&spec, prompt).await?;
    // read a few tokens ...
} // dropped here; run is cancelled
```

## See also

- [Human-in-the-loop](human-in-the-loop.md)
- [API reference](api-reference.md)
