# Configuring Redaction

## Span Policy

`SpanPolicy` controls which span attributes are exported. Configure it via:

```rust
use ancora_telpriv::span_policy::SpanPolicy;
use ancora_telpriv::classification::DataClass;

let policy = SpanPolicy {
    redact_at_or_above: DataClass::Sensitive,
    drop_prefixes: vec!["prompt.".to_string()],
};
```

- `redact_at_or_above`: attributes at this class or higher are replaced with
  `[REDACTED]`.
- `drop_prefixes`: attributes whose name starts with any of these prefixes are
  dropped entirely (not even the key is exported).

## Log Policy

`LogPolicy` controls log record export. Configure it via:

```rust
use ancora_telpriv::log_policy::LogPolicy;
use ancora_telpriv::log_policy::LogLevel;

let policy = LogPolicy {
    min_export_level: LogLevel::Info,
    scrub_message: true,
    sensitive_fields: vec!["email".to_string(), "ip".to_string()],
};
```

## Eval Policy

`EvalPolicy` controls what eval data reaches the telemetry sink:

```rust
use ancora_telpriv::eval_policy::EvalPolicy;

let policy = EvalPolicy {
    allow_prompt_export: false,  // default: off
    allow_completion_export: false,
    scrub_metadata: true,
};
```

Set `allow_prompt_export: true` only in controlled environments where you have
a data processing agreement in place.

## Opt-In Features

Use `OptInRegistry::from_env_str` to parse a comma-separated list of features
from an environment variable:

```
ANCORA_TELEMETRY_OPT_IN=prompt_capture,eval_text_export
```

Available tokens: `prompt_capture`, `completion_capture`, `user_id_correlation`,
`eval_text_export`, `full_stack_traces`.

## Allowlist

To extend the set of safe attributes, add exact names or prefixes:

```rust
use ancora_telpriv::allowlist::Allowlist;

let mut al = Allowlist::default_safe();
al.add_exact("app.tenant_id");
al.add_prefix("custom.safe.");
```
