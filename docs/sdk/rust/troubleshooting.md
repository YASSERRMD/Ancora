# Troubleshooting (Rust)

## `error[E0308]: mismatched types` on `RunEvent`

Ensure you are matching exhaustively against `RunEvent` variants.
The compiler requires all arms or a wildcard:

```rust
match ev {
    RunEvent::Token { token } => print!("{}", token),
    RunEvent::Completed { output } => println!("{}", output),
    _ => {} // required if you do not handle all variants
}
```

## `connection refused` to Ollama

```
Error: connection refused (os error 111)
```

Start the Ollama server before running your program:

```bash
ollama serve
```

Check `ANCORA_MODEL_URL` points to the correct host and port.

## `anyhow::Error: model not found`

Pull the model first:

```bash
ollama pull llama3
```

List available models: `ollama list`.

## Compile error: feature `sqlite` not found

Enable the feature in `Cargo.toml`:

```toml
ancora-core = { git = "...", features = ["sqlite"] }
```

## Run hangs and never completes

- Check that the model is loaded in Ollama (`ollama ps`).
- Increase the HTTP timeout in `RuntimeOptions`:

```rust
let rt = Runtime::with_options(RuntimeOptions {
    http_timeout: Some(std::time::Duration::from_secs(600)),
    ..Default::default()
})?;
```

## `PolicyViolationError` in tests

Tests that specify a `PolicySpec` will fail if the test environment is not
in an allowed region. Remove the policy or use `allow_regions(vec![])` (allow
all) in test builds:

```rust
#[cfg(test)]
let policy = PolicySpec::builder().build(); // no restrictions
```

## See also

- [Configuration](configuration.md)
- [Testing](testing.md)
