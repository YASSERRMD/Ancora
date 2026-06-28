# Writing Extensions in Python

This guide covers how to author an Ancora tool extension in Python. Python
extensions are loaded at runtime via the `ancora-py` PyO3 binding and
validated against the Rust interop kit.

## Base class

```python
from ancora import ToolExtension, ToolMeta, ExtensionError

class EchoTool(ToolExtension):
    @property
    def meta(self) -> ToolMeta:
        return ToolMeta(
            name="py_echo",
            description="Echoes the input message.",
            version="1.0.0",
        )

    def execute(self, args: dict) -> object:
        message = args.get("message")
        if message is None:
            raise ExtensionError("'message' argument is required")
        return f"[python] {message}"

    def health_check(self) -> None:
        pass  # raise ExtensionError if unhealthy
```

## The `@ancora_tool` decorator

For function-based extensions use the decorator:

```python
from ancora import ancora_tool

@ancora_tool(name="py_upper", description="Uppercases text.", version="1.0.0")
def uppercase(args: dict) -> str:
    text = args.get("text", "")
    return text.upper()
```

## Manifest

The manifest JSON emitted when an extension is first loaded:

```json
{
  "name": "py_echo",
  "description": "Echoes the input message.",
  "version": "1.0.0",
  "class_path": "mypackage.tools.EchoTool",
  "requirements": []
}
```

All four required fields (`name`, `description`, `version`, `class_path`) must
be non-empty; the Rust bridge validates them via `validate_manifest`.

## Packaging

Structure your extension as a standard Python package:

```
my_ancora_ext/
    __init__.py
    tools.py          # contains your ToolExtension subclasses
    ancora_manifest.json
```

## Testing

Run the parity checks from Python:

```python
from ancora.testing import InteropKit

results = InteropKit.run_all(EchoTool())
for r in results:
    assert r.passed, f"{r.name}: {r.message}"
```
