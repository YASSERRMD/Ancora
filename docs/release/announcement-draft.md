# Announcement Draft -- Ancora 0.6.0

## LinkedIn post

We are shipping Ancora 0.6.0 today.

Ancora is a local-first, privacy-respecting multi-agent runtime with a deterministic
replay guarantee. Every agent run is journalled, replayable, and auditable offline.

What is in 0.6.0:

- 100+ offline test files covering determinism, security, policy, reliability, chaos, load,
  coverage gating, documentation audit, example parity, and performance benchmarks.
- All six SDK languages (Rust, Go, Python, TypeScript, .NET, Java) at feature parity.
- Ten Chinese providers (Qwen, GLM, DeepSeek, Kimi, MiniMax, StepFun, ERNIE, Hunyuan,
  Doubao, MiMo) running from recorded fixtures with correct cost accounting and residency tags.
- All 11 vector store backends conformance-tested (inmemory, sqlite, pgvector, qdrant,
  weaviate, milvus, lancedb, chroma, pinecone, vespa, redis).
- Signed binaries, SBOM, and publish dry-runs for crates.io, PyPI, npm, NuGet, and Maven.

Everything runs offline by default. No live API calls in CI.

GitHub: https://github.com/YASSERRMD/Ancora

## Repository release description

Ancora 0.6.0 -- complete offline test program, six SDK languages at parity, ten Chinese
providers, eleven vector backends, signed artifacts, and SBOM.

See CHANGELOG.md for the full list of additions.
See docs/release/ for the platforms matrix, provider coverage matrix, feature parity
matrix, upgrade guide, security advisory process, and versioning policy.
