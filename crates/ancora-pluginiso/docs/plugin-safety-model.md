# Plugin Safety Model

## Overview

The ancora-pluginiso crate defines the isolation contract between Ancora host
processes and third-party plugins. Every plugin runs inside a sandbox that
enforces the following constraints:

- Resource limits (CPU, memory, threads, file descriptors)
- Network policy (allowlist / denylist of outbound connections)
- Filesystem policy (path-scoped read/write permissions)
- Capability grants (named permissions beyond basic compute)
- Crash isolation (plugin faults do not propagate to the host)
- Signature verification (only signed, trusted plugins may load)
- Data-residency enforcement (data stays in permitted geographic regions)

## Design Principles

### Least privilege

Every plugin starts with the most restrictive defaults: no network access, no
filesystem access, no capabilities, and a strict signature requirement. Host
operators explicitly grant permissions rather than restricting them after the
fact.

### Defense in depth

Multiple independent isolation layers are combined. A plugin that bypasses one
layer (e.g., a Wasm sandbox escape) still faces OS-level process isolation,
rlimits, and the host's IPC gate.

### Crash isolation by default

Plugin crashes are contained. The host records the crash in the audit log,
marks the plugin as faulted, and returns an error to callers. The host process
continues serving other workloads without interruption.

### Auditability

Every lifecycle event - load, unload, crash, policy violation, signature check
result - is appended to an immutable audit log that can be forwarded to a
SIEM or compliance store.

## Sandbox Configuration

A `Sandbox` struct bundles all isolation constraints for a single plugin
instance. It is passed at instantiation time and cannot be modified
afterwards.

```rust
let sandbox = Sandbox::new(
    "my-plugin-v1",
    RuntimeKind::Wasm,
    ResourceLimits::default(),
    NetworkPolicy::deny_all(),
    FilesystemPolicy::deny_all(),
    CapabilityGrants::none(),
    CrashIsolationMode::Isolated,
    SignaturePolicy::Required,
);
```

## Audit Trail

All events are appended to an `AuditLog`:

- `EventKind::Loaded` - plugin successfully loaded
- `EventKind::Unloaded` - plugin cleanly shut down
- `EventKind::Crashed` - plugin faulted
- `EventKind::PolicyViolation` - blocked network/filesystem/resource access
- `EventKind::SignatureVerified` / `SignatureFailed` - signature check result
