# Human-in-the-Loop (Rust)

## Suspend and resume

Ancora runs can pause at a defined suspension point and wait for human input.

```rust
use ancora_core::{AgentSpec, Runtime, RunEvent, SuspendSignal};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let rt = Runtime::new()?;

    let spec = AgentSpec::builder()
        .model("llama3")
        .instructions(
            "Draft an email. When ready to send, call the `send_email` \
             tool. Ask the user to confirm before sending.",
        )
        .tools(vec![send_email_tool()])
        .build();

    let mut run = rt.run(&spec, "Write an email to the team about the Q3 review.").await?;

    loop {
        match run.next().await? {
            Some(RunEvent::Suspended { reason }) => {
                eprintln!("Paused: {}", reason);
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
                run.resume(input.trim()).await?;
            }
            Some(RunEvent::Completed { output }) => {
                println!("{}", output);
                break;
            }
            Some(_) => {}
            None => break,
        }
    }

    Ok(())
}
```

## Triggering suspension from a tool

A tool handler can return a `SuspendSignal` to pause the run:

```rust
.handler(|_args| async {
    Err(SuspendSignal::new("Please confirm: send this email? (yes/no)").into())
})
```

The run emits `RunEvent::Suspended { reason }` and blocks until `run.resume(input)` is called.

## Multi-turn conversation

```rust
let mut run = rt.run(&spec, "Help me write a haiku.").await?;

loop {
    match run.next().await? {
        Some(RunEvent::Suspended { .. }) => {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line)?;
            run.resume(line.trim()).await?;
        }
        Some(RunEvent::Completed { output }) => { println!("{}", output); break; }
        Some(_) => {}
        None => break,
    }
}
```

## See also

- [Streaming](streaming.md)
- [Durability](durability.md)
