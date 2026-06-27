# Verifier Pattern (Rust)

## Primary and verifier agents

```rust
use ancora_core::{AgentSpec, Runtime, RunEvent};
use tokio::task::JoinSet;

async fn run_to_output(
    rt: &Runtime,
    spec: &AgentSpec,
    prompt: &str,
) -> anyhow::Result<String> {
    let mut run = rt.run(spec, prompt).await?;
    while let Some(ev) = run.next().await? {
        if let RunEvent::Completed { output } = ev {
            return Ok(output);
        }
    }
    Ok(String::new())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let rt = Runtime::new()?;

    let primary = AgentSpec::builder()
        .model("llama3")
        .instructions("Answer the question.")
        .build();

    let verifier = AgentSpec::builder()
        .model("llama3")
        .instructions(
            "You receive a question and a proposed answer. \
             Reply 'CORRECT' or 'INCORRECT: <reason>'.",
        )
        .build();

    let question = "What is 17 * 23?";
    let answer = run_to_output(&rt, &primary, question).await?;

    let verify_prompt = format!("Question: {}\nAnswer: {}", question, answer);
    let verdict = run_to_output(&rt, &verifier, &verify_prompt).await?;

    println!("Answer : {}", answer);
    println!("Verdict: {}", verdict);
    Ok(())
}
```

## N-verifier consensus with `JoinSet`

```rust
let n = 3;
let mut set = JoinSet::new();

for _ in 0..n {
    let rt = rt.clone();
    let spec = verifier.clone();
    let prompt = verify_prompt.clone();
    set.spawn(async move { run_to_output(&rt, &spec, &prompt).await });
}

let mut correct = 0usize;
while let Some(res) = set.join_next().await {
    if res??.starts_with("CORRECT") { correct += 1; }
}

if correct >= 2 {
    println!("Consensus: CORRECT");
}
```

## See also

- [Multi-agent graphs](multi-agent.md)
- [Concurrency](concurrency.md)
