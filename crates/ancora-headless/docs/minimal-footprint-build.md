# Minimal Footprint Build

`ancora-headless` is designed to run on inference-OS hardware with constrained
resources. The following guidelines keep the footprint within the default targets.

## Default Targets

| Metric | Target |
|--------|--------|
| Binary size | ≤ 50 MB |
| Runtime dependencies | ≤ 50 |
| RSS at boot | ≤ 256 MB |
| Total disk (binary + assets) | ≤ 512 MB |

## Cargo Features

Only `std`, `serde`, and `serde_json` are mandatory. Heavy optional crates
(async runtimes, TLS, protobuf, observability SDKs) must be placed behind
feature flags and excluded from the default headless build.

## Build Flags

```sh
# Strip debug symbols
RUSTFLAGS="-C strip=symbols" cargo build --release -p ancora-headless

# Link-time optimisation
RUSTFLAGS="-C lto=fat -C codegen-units=1" cargo build --release -p ancora-headless
```

## Checking Footprint

```rust
use ancora_headless::footprint::{FootprintMeasurement, FootprintTarget, check_footprint};

let m = FootprintMeasurement::new("release", binary_bytes, dep_count, rss_mb, disk_mb);
let t = FootprintTarget::default();
assert_eq!(check_footprint(&m, &t), FootprintStatus::WithinTarget);
```

## Dependency Audit

Run `cargo tree -p ancora-headless` after every dependency change and verify the
count stays within the manifest limit. Use `FootprintManifest` to track the
mandatory vs optional split in CI.
