# Ancora Python SDK

Python bindings for the Ancora agent runtime via PyO3 and maturin.

## Install

```bash
pip install maturin
maturin develop
```

## Usage

```python
import ancora

rt = ancora.Runtime()
print(ancora.version())
rt.free()
```

Or with a context manager:

```python
import ancora

with ancora.Runtime() as rt:
    print(rt)
```
