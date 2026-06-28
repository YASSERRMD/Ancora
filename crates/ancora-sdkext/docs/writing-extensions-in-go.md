# Writing Extensions in Go

This guide covers how to author an Ancora tool extension in Go and expose it
to the Rust runtime via the `GoExtensionAdapter`.

## Interface contract

Your Go extension must implement the `ToolExtension` interface:

```go
package ancora

type ToolMeta struct {
    Name        string
    Description string
    Version     string
}

type ToolExtension interface {
    Meta() ToolMeta
    Execute(args map[string]any) (any, error)
    HealthCheck() error
}
```

## Example implementation

```go
package myext

import "errors"

type EchoTool struct{}

func (e EchoTool) Meta() ancora.ToolMeta {
    return ancora.ToolMeta{
        Name:        "go_echo",
        Description: "Echoes the input message.",
        Version:     "1.0.0",
    }
}

func (e EchoTool) Execute(args map[string]any) (any, error) {
    msg, ok := args["message"].(string)
    if !ok {
        return nil, errors.New("'message' argument is required")
    }
    return "[go] " + msg, nil
}

func (e EchoTool) HealthCheck() error {
    return nil
}
```

## Exposing the extension via the ABI

Compile your Go extension as a shared library and export the `AncoraMeta`,
`AncoraExecute`, and `AncoraHealthCheck` C-exported symbols. The Rust bridge
loads the shared library and wraps it in a `GoExtensionAdapter`.

## Testing

The Rust-side interop kit checks:

1. `meta()` returns non-empty name, description, and version.
2. `health_check()` returns `Ok(())`.
3. `execute()` does not panic on an empty argument map.
4. `execute()` does not panic on an unrecognised argument key.

Your Go tests should mirror these checks using the Go standard `testing` package.

## Registration

Register the adapter once the shared library is loaded:

```rust
use ancora_sdkext::registration::{ExtensionRegistry, register_go_extension};
use std::sync::Arc;

let registry = ExtensionRegistry::new();
register_go_extension(&registry, Arc::new(my_go_adapter)).unwrap();
```
