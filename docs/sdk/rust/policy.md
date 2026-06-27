# Policy (Rust)

## Data-residency rules with `PolicySpec`

```rust
use ancora_core::{AgentSpec, PolicySpec};

let policy = PolicySpec::builder()
    .allow_regions(vec!["eu-west-1".into(), "eu-central-1".into()])
    .deny_providers(vec!["openai".into()])
    .max_write_tools(0)
    .build();

let spec = AgentSpec::builder()
    .model("llama3")
    .instructions("Summarise this EU customer support ticket.")
    .policy(policy)
    .build();
```

## `PolicySpec` fields

| Field | Type | Description |
|-------|------|-------------|
| `allow_regions` | `Vec<String>` | Inference must happen in one of these regions |
| `deny_providers` | `Vec<String>` | These provider names are blocked |
| `max_write_tools` | `Option<u32>` | Maximum number of `Write`-class tool calls per run |
| `require_encryption` | `bool` | All transport must use TLS |

## Handling `PolicyViolationError`

```rust
use ancora_core::PolicyViolationError;

match rt.run(&spec, prompt).await {
    Err(e) if e.is::<PolicyViolationError>() => {
        eprintln!("Policy blocked: {}", e);
    }
    Err(e) => return Err(e.into()),
    Ok(run) => { /* process events */ }
}
```

## Per-tool policy

Combine policy with `EffectClass::Write` to guard mutations:

```rust
let write_tool = ToolSpec::builder()
    .name("write_database")
    .effect(EffectClass::Write)
    // ...
    .build();

let policy = PolicySpec::builder()
    .max_write_tools(1) // allow at most 1 write per run
    .build();
```

## See also

- [Tools](tools.md)
- [Providers](providers.md)
