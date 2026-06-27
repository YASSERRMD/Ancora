# Troubleshooting (Python)

## `OSError: libancora_ffi.so: cannot open shared object file`

The native library is not on the dynamic linker path.

**Fix**:

```bash
export LD_LIBRARY_PATH=/path/to/target/release:$LD_LIBRARY_PATH
```

Or install the pre-built wheel which bundles the library:

```bash
pip install ancora
```

## `ModuleNotFoundError: No module named 'ancora._cffi_backend'`

The CFFI extension has not been compiled.

**Fix**: rebuild the Python extension:

```bash
pip install -e sdk/python
```

## `ConnectionRefusedError` when calling Ollama

Ollama is not running.

**Fix**:

```bash
ollama serve
```

## `RuntimeError: model not found`

The model weight has not been pulled.

**Fix**:

```bash
ollama pull llama3
```

## `PolicyViolationError: max_write_tools exceeded`

The agent attempted more write-effect tool calls than the policy allows.

**Fix**: increase `max_write_tools` in `PolicySpec` or reduce write tool calls.

## `ValidationError` from Pydantic on structured output

The model returned JSON that does not match the schema.

**Fix**: add a retry in your code:

```python
for attempt in range(3):
    result = rt.run(spec, prompt)
    try:
        output = result.parse(MyModel)
        break
    except Exception:
        if attempt == 2:
            raise
```

## See also

- [Install](install.md)
- [Durability](durability.md)
