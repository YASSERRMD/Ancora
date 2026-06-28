# Extension Ergonomics Overview

The `ancora-sdkext` crate provides everything needed to author, register, and
validate Ancora tool extensions across six supported languages: Rust, Go,
Python, TypeScript, .NET, and Java.

## Design principles

- **Single interface surface** - every language implements the same three-method
  contract: `meta()`, `execute()`, and `health_check()`.
- **No network calls in tests** - the interop kit runs entirely offline; adapters
  inject closures instead of real language runtimes.
- **Fail loudly at registration** - duplicate names are rejected immediately so
  mis-wired extensions are caught at startup, not at call time.
- **Parity matrix** - the `ParityMatrix` struct records per-language pass/fail
  results so CI can assert that all six languages reach full parity.

## Architecture

```
┌─────────────────────────────────────────────────────┐
│                  ExtensionRegistry                   │
│  (thread-safe, keyed by extension name)              │
└──────┬─────┬──────┬──────┬──────┬──────┬───────────┘
       │     │      │      │      │      │
      Rust  Go   Python   TS  .NET  Java
       │     │      │      │      │      │
    (direct) (GoExt) (PyExt) (TsExt) (DotNetExt) (JavaExt)
              Adapter Adapter Adapter  Adapter    Adapter
```

Every adapter presents a `ToolExtension` trait object to the registry regardless
of the source language.

## Interop kit checks

| Check | What it verifies |
|-------|-----------------|
| `meta_non_empty` | name, description, and version are all non-empty |
| `health_ok` | `health_check()` returns `Ok(())` |
| `execute_returns_value` | `execute({})` does not panic |
| `invalid_arg_no_panic` | `execute({"__invalid__": ...})` does not panic |

## Quick reference

| Language | Adapter type | Registration helper |
|----------|-------------|---------------------|
| Rust | direct `ToolExtension` impl | `register_rust_extension` |
| Go | `GoExtensionAdapter` | `register_go_extension` |
| Python | `PyExtensionAdapter` | `register_python_extension` |
| TypeScript | `TsExtensionAdapter` | `register_typescript_extension` |
| .NET | `DotNetExtensionAdapter` | `register_dotnet_extension` |
| Java | `JavaExtensionAdapter` | `register_java_extension` |
