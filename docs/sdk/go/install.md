# Install and Prerequisites

## Prerequisites

- Go 1.22 or later
- CGo enabled (`CGO_ENABLED=1`)
- C compiler (gcc or clang)
- The `ancora_ffi` native library on `LD_LIBRARY_PATH` (Linux) or
  `DYLD_LIBRARY_PATH` (macOS)

## Building the native library

```bash
git clone https://github.com/YASSERRMD/Ancora
cd Ancora
cargo build --release -p ancora-ffi
```

The library is at `target/release/libancora_ffi.so` (Linux) or
`target/release/libancora_ffi.dylib` (macOS).

## Installing the Go module

```bash
go get ancora.io/sdk@latest
```

Or add to `go.mod`:

```
require ancora.io/sdk v0.1.0
```

## Environment setup

```bash
export CGO_ENABLED=1
export LD_LIBRARY_PATH=/path/to/ancora/target/release:$LD_LIBRARY_PATH
```

For macOS replace `LD_LIBRARY_PATH` with `DYLD_LIBRARY_PATH`.

## Verify

```go
package main

import (
    "fmt"
    "ancora.io/sdk"
)

func main() {
    rt, err := ancora.NewRuntime()
    if err != nil {
        panic(err)
    }
    defer rt.Close()
    fmt.Println("Ancora runtime ready")
}
```

```bash
go run main.go
# Ancora runtime ready
```
