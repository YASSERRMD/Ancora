# Install (Python)

## Requirements

- Python 3.10 or later
- A C compiler (GCC or Clang) to build CFFI extension wheels
- A Rust toolchain (for building the native library from source, or use a
  pre-built binary)

## Install from PyPI

```bash
pip install ancora
```

The wheel bundles a pre-built native library for the most common platforms
(`linux-x86_64`, `linux-aarch64`, `macos-arm64`, `macos-x86_64`). If no
pre-built wheel matches your platform, pip will fall back to building from
source.

## Build from source

```bash
# 1. Build the native Rust library
cargo build --release -p ancora-ffi

# 2. Set environment variables so CFFI can find the headers and library
export ANCORA_LIB_DIR="$(pwd)/target/release"
export ANCORA_INCLUDE_DIR="$(pwd)/crates/ancora-ffi/include"

# 3. Install in editable mode
pip install -e sdk/python
```

## Virtual environment

```bash
python -m venv .venv
source .venv/bin/activate
pip install ancora
```

## Verify

```python
import ancora
print(ancora.__version__)
```

## Runtime prerequisites

Set the model endpoint if not using the default local Ollama server:

```bash
export ANCORA_MODEL_URL="http://127.0.0.1:11434"   # Ollama (default)
# export ANCORA_MODEL_URL="https://api.anthropic.com/v1"  # Anthropic
```

## See also

- [Quickstart](quickstart.md)
- [Providers](providers.md)
