# Tools (Rust)

## Registering a closure as a tool

```rust
use ancora_core::{AgentSpec, Runtime, RunEvent, ToolSpec, ToolInputSchema, EffectClass};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let rt = Runtime::new()?;

    let weather_tool = ToolSpec::builder()
        .name("get_weather")
        .description("Return current weather for a city.")
        .schema(ToolInputSchema {
            type_: "object".into(),
            properties: json!({
                "city": { "type": "string", "description": "City name" }
            }),
            required: vec!["city".into()],
        })
        .effect(EffectClass::Read)
        .handler(|args: Value| async move {
            let city = args["city"].as_str().unwrap_or("unknown");
            Ok(json!({ "temp": "22C", "condition": "sunny", "city": city }))
        })
        .build();

    let spec = AgentSpec::builder()
        .model("llama3")
        .instructions("Answer questions about weather.")
        .tools(vec![weather_tool])
        .build();

    let mut run = rt.run(&spec, "What is the weather in Tokyo?").await?;
    while let Some(ev) = run.next().await? {
        if let RunEvent::Completed { output } = ev {
            println!("{}", output);
        }
    }
    Ok(())
}
```

## `EffectClass`

| Variant | Description |
|---------|-------------|
| `EffectClass::None` | Pure computation, no side effects |
| `EffectClass::Read` | Reads external state (network, disk) |
| `EffectClass::Write` | Mutates external state |

Policy rules can block `Write`-class tools by region.

## Async tool handlers

Tool handlers must return `impl Future<Output = anyhow::Result<Value>>`.
Use `async move` closures and `.await` external calls inside them.

## Multiple tools

```rust
let spec = AgentSpec::builder()
    .model("llama3")
    .instructions("You have weather and time tools.")
    .tools(vec![weather_tool, time_tool, calendar_tool])
    .build();
```

## See also

- [Structured output](structured-output.md)
- [Policy](policy.md)
