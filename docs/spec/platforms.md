# Supported Platforms and Versions

## Rust crates

| Crate | MSRV | Tier |
|-------|------|------|
| `ancora-core` | Rust 1.75 | Tier 1 |
| `ancora-proto` | Rust 1.75 | Tier 1 |
| `ancora-ffi` | Rust 1.75 | Tier 1 |
| `ancora-tools` | Rust 1.75 | Tier 1 |
| `ancora-policy` | Rust 1.75 | Tier 1 |
| `ancora-grpc` | Rust 1.75 | Tier 1 |

**Tier 1**: tested in CI on every PR; breakage blocks merge.

## Language bindings

| Binding | Minimum version | Package |
|---------|----------------|---------|
| Go | 1.22 | `sdk/go` |
| Python | 3.9 | `sdk/python` (PyPI: `ancora`) |
| TypeScript | Node.js 18 / TS 5 | `sdk/ts` (npm: `@ancora/sdk`) |
| .NET | .NET 8 | `sdk/dotnet` (NuGet: `Ancora.Sdk`) |
| Java | Java 17 (LTS) | `sdk/java` (Maven: `com.ancora:ancora-sdk`) |

## Operating systems

| OS | Tier |
|----|------|
| Linux x86-64 (glibc >= 2.28) | Tier 1 |
| macOS 13+ (Apple Silicon and Intel) | Tier 1 |
| Windows 11 / Server 2022 (MSVC) | Tier 2 |
| Linux ARM64 | Tier 2 |
| Linux x86-64 (musl) | Tier 2 |

**Tier 1**: tested in CI on every PR.
**Tier 2**: best-effort; known to work but not in CI on every PR.

## CI matrix

| Job | OS | Rust | Language |
|-----|----|------|----------|
| `build.yml` | ubuntu-22.04 | stable | -- |
| `test.yml` | ubuntu-22.04 | stable | -- |
| `clippy.yml` | ubuntu-22.04 | stable | -- |
| `fmt.yml` | ubuntu-22.04 | stable | -- |
| `abi-check.yml` | ubuntu-22.04 | stable | -- |
| `ts-ci.yml` | ubuntu-22.04 | stable | Node.js 20 |
| `py-smoke.yml` | ubuntu-22.04 | stable | Python 3.11 |
| `dotnet-ci.yml` | ubuntu-22.04 | stable | .NET 8 |
| `java-ci.yml` | ubuntu-22.04 | stable | Java 17 |
| `xlang-conformance.yml` | ubuntu-22.04 | stable | Go, Python |
| `docs.yml` | ubuntu-22.04 | -- | -- |
| `publish-dry-run.yml` | ubuntu-22.04 | stable | all |

## Native artifacts

Pre-built shared libraries (`.so`, `.dylib`, `.dll`) are produced by
`native-artifacts.yml` for the three Tier 1 operating system / architecture
combinations. Language bindings that do not ship a Rust toolchain bundle
these prebuilt libraries.

## Version policy

- **Patch releases** (0.1.x): bug fixes, no API changes.
- **Minor releases** (0.x.0): additive API changes; backwards-compatible.
- **Major releases** (x.0.0): breaking changes with migration guides.

The Rust workspace, all language bindings, and the FFI C header are versioned
together and released as a single coordinated tag (`vX.Y.Z`). The release CI
(`release.yml`) verifies version consistency before creating the GitHub release.
