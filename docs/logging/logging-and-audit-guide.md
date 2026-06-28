# Logging and Audit Guide

## Structured logging

All log output from Ancora uses JSON format. Each record includes:

- `timestamp_secs` - Unix timestamp
- `level` - one of Trace, Debug, Info, Warn, Error
- `module` - originating crate or module
- `message` - human-readable message
- `run_id`, `tenant_id`, `trace_id` - correlation identifiers (optional)
- `fields` - additional structured data (never contains secrets)

```rust
use ancora_logging::{LogRecord, LogLevel};

let rec = LogRecord::new(LogLevel::Info, "ancora-worker", "run started", now)
    .with_correlation(run_id, tenant_id, trace_id);
println!("{}", rec.to_json());
```

## Secret redaction

Before emitting any log record to external systems, pass through `redact_json`:

```rust
use ancora_logging::redact_json;
let safe = redact_json(&rec.to_json());
```

Fields whose names contain: `api_key`, `token`, `password`, `secret`, `credential`, `private_key`, or `auth` are replaced with `[REDACTED]`.

## Log levels per module

```rust
use ancora_logging::{LevelConfig, LogLevel};

let mut cfg = LevelConfig::new(LogLevel::Info);
cfg.set_module("ancora-worker", LogLevel::Debug);
assert!(cfg.is_enabled("ancora-worker", &LogLevel::Debug));
```

## Sampling for high-volume logs

Use `Sampler` to reduce log volume for frequent trace-level events:

```rust
use ancora_logging::Sampler;

let mut s = Sampler::new(100); // sample 1 in 100
if s.should_sample() {
    emit_log(record);
}
```

## Audit channel

The audit channel is separate from the application log stream. Audit events are signed and append-only.

```rust
use ancora_logging::{AuditChannel, AuditEvent, AuditEventKind};

let mut audit = AuditChannel::new();
audit.append(AuditEvent::new(
    now,
    AuditEventKind::PolicyDecision,
    tenant_id,
    actor,
    resource,
    "allowed",
    signing_key,
));
```

## Verification

```rust
assert!(event.verify(signing_key));
```

A tampered event will fail verification. Persist the signing key in a secret store (not inline).
