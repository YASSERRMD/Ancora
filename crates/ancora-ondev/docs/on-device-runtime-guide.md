# On-Device Runtime Guide

`ancora-ondev` provides a minimal, offline-first agent runtime designed for
ARM-class CPUs and mobile operating systems.

## Supported Targets

| Target | Triple | Notes |
|--------|--------|-------|
| ARM64 Linux | `aarch64-unknown-linux-musl` | Static musl binary, runs on Pi 4, Jetson |
| ARM32 Linux | `armv7-unknown-linux-musleabihf` | Hard-float, Cortex-A8+ |
| Android | `aarch64-linux-android` | NDK r25+, API level 21+ |
| iOS | `aarch64-apple-ios` | Xcode 15+, iOS 14+ |

## Quick Start

### Build for ARM64 Linux

```bash
rustup target add aarch64-unknown-linux-musl
cargo build --target aarch64-unknown-linux-musl \
    --profile ondev \
    --no-default-features \
    --features arm64
```

### Run Tests Locally

```bash
cargo test -p ancora-ondev
```

All tests run offline; no network access is required.

## Crate Structure

```
ancora-ondev/
  src/
    build_profile.rs   # Compile-time optimisation settings
    targets.rs         # Target triple definitions
    features.rs        # Feature flag registry
    journal.rs         # Embedded SQLite journal
    memory.rs          # Embedded LanceDB vector store
    inference.rs       # Local-only inference engine
    perf.rs            # Cold-start and memory footprint measurement
    docs_meta.rs       # Documentation topic registry
    tests/             # Integration tests
  docs/                # This documentation
```

## Design Principles

1. **Offline-first** -- every code path works without a network connection.
2. **Small footprint** -- the default build profile targets a binary under 5 MiB.
3. **Safe by default** -- `local_only = true` is the default; remote backends
   must be explicitly opted in (and only when local-only mode is disabled).
4. **No async required** -- all APIs are synchronous; no Tokio runtime is
   bundled, which reduces binary size and cold-start time.

## Build Profile

Add the following to your workspace `Cargo.toml` to reproduce the recommended
on-device build profile:

```toml
[profile.ondev]
inherits    = "release"
opt-level   = "z"
panic       = "abort"
lto         = true
strip       = true
codegen-units = 1
```

Then build with `--profile ondev`.
