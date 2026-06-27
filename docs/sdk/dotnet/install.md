# Install (.NET)

## Requirements

- .NET 8 SDK or later
- A Rust toolchain to build the native library (or use a pre-built binary)
- The native library (`libancora_ffi.so` / `ancora_ffi.dll`) must be on the
  library path at runtime

## Install from NuGet

```bash
dotnet add package Ancora
```

Pre-built native binaries are bundled for common platforms
(`linux-x64`, `linux-arm64`, `osx-arm64`, `osx-x64`, `win-x64`).

## Build from source

```bash
# 1. Build the native Rust library
cargo build --release -p ancora-ffi

# 2. Copy it alongside your binary
cp target/release/libancora_ffi.so path/to/publish/

# 3. Add the SDK package
dotnet add package Ancora
```

## Runtime library path

```bash
# Linux
export LD_LIBRARY_PATH=/path/to/native:$LD_LIBRARY_PATH

# macOS
export DYLD_LIBRARY_PATH=/path/to/native:$DYLD_LIBRARY_PATH

# Windows
set PATH=C:\path\to\native;%PATH%
```

## Runtime prerequisites

```bash
export ANCORA_MODEL_URL="http://127.0.0.1:11434"   # Ollama (default)
```

## Verify

```csharp
using Ancora;
Console.WriteLine(AncoraSdk.Version);
```

## See also

- [Quickstart](quickstart.md)
- [Providers](providers.md)
