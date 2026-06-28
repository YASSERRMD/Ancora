# Customizing Ancora Presets

## Adding capabilities

Use `with_capability()` to add capabilities that the built-in preset does not
include:

```rust
use ancora_preset::{assemble, research_assistant, Capability};

let preset = research_assistant()
    .with_capability(Capability::CostControl);

let spec = assemble(&preset).expect("valid");
```

## Applying key-value overrides

Overrides attach arbitrary key-value pairs to the preset.  They are encoded
into the `system_prompt` as `override:key=value` lines and do not affect
`capabilities`, `air_gap`, or `residency`.

```rust
use ancora_preset::{apply_overrides, get_override, research_assistant};

let preset = research_assistant()
    .with_override("max_citations", "50");

// Or apply a batch of overrides at runtime:
let updated = apply_overrides(
    preset,
    vec![
        ("max_citations".to_string(), "100".to_string()),
        ("timeout_secs".to_string(), "120".to_string()),
    ],
);

assert_eq!(get_override(&updated, "max_citations"), Some("100"));
```

`apply_overrides()` replaces the value for an existing key and appends new
keys.  The original preset is consumed and a modified copy is returned.

## Building a custom preset from scratch

```rust
use ancora_preset::{assemble, AirGapPolicy, Capability, PresetDescriptor};

let preset = PresetDescriptor::new("my-preset", "Description here")
    .with_capability(Capability::Planning)
    .with_capability(Capability::Guardrails)
    .with_capability(Capability::Memory)
    .with_override("budget_tokens", "4096");

let spec = assemble(&preset).expect("custom preset assembled");
```

## Locking a preset

A locked preset signals that no additional capabilities may be introduced at
runtime.  The orchestrator SHOULD enforce this constraint.

```rust
let locked = PresetDescriptor::new("strict", "locked config")
    .with_capability(Capability::Planning)
    .with_locked(true);
```

The `locked:true` flag appears in the assembled `system_prompt`.

## Preset override precedence

1. Preset defaults (fields set by the preset constructor)
2. `.with_override()` calls on the builder
3. `apply_overrides()` batch -- replaces existing keys, appends new ones

`capabilities`, `air_gap`, and `residency` are never modified by overrides.
