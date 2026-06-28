# ancora-appspar: Sample App Index

## Crate purpose

ancora-appspar verifies that every language SDK ships a sample agent app that
is functionally at parity with all other languages.

## Modules

| Module        | Description                                    |
|---------------|------------------------------------------------|
| `go_app`      | Go sample app model                            |
| `python_app`  | Python sample app model                        |
| `ts_app`      | TypeScript sample app model                    |
| `dotnet_app`  | .NET/C# sample app model                       |
| `java_app`    | Java sample app model                          |
| `rust_app`    | Rust sample app model                          |
| `parity`      | Feature parity checker across all languages    |
| `polyglot`    | A2A polyglot router for cross-language agents  |

## Documentation

- [Parity matrix](parity-matrix.md) - feature-by-language table
- [Per-language notes](per-language-notes.md) - SDK, constructor, trace format
- [Cross-language guide](cross-language-guide.md) - interaction model and A2A
- [CI](ci.md) - what CI checks and how to extend

## Quick start

```rust
use ancora_appspar::go_app::GoApp;
let app = GoApp::new("my-agent");
let trace = app.run("hello").unwrap();
println!("{:?}", trace);
```
