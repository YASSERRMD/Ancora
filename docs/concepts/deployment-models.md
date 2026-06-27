# Deployment Models

Ancora supports four deployment models, from a single binary on a laptop to
a multi-replica cluster. All deployment models share the same agent code.

## 1. Single binary (edge / offline)

- Compile with `cargo build --release` (Rust) or use a pre-built SDK.
- Embed LanceDB for vector storage.
- Use SQLite for the journal.
- Inference via Ollama or llama.cpp on the same machine.
- Zero external dependencies. Works air-gapped.

**Use case**: IoT, air-gapped secure environments, developer laptops.

## 2. Single server

- Deploy the `ancora-cli` binary or a gRPC-enabled service.
- Journal backed by PostgreSQL + pgvector.
- Inference via Ollama on the same server or a local GPU cluster.
- Optional remote model provider for overflow capacity.

**Use case**: team-shared agent service, SME deployments.

## 3. Multi-replica cluster

- Multiple `ancora-core` replicas behind a load balancer.
- Shared PostgreSQL journal (or Milvus for vector).
- Stateless replicas -- any replica can replay any run.
- Horizontal scaling for throughput.

**Use case**: production SaaS, enterprise internal platforms.

## 4. Air-gapped sovereign deployment

- No internet access allowed.
- All models are local (GGUF weights copied via secure media).
- Journal on an air-gapped PostgreSQL instance.
- Policy rules block any remote provider calls.

**Use case**: government, defense, financial services with strict data
residency requirements.

## Choosing a model

| Requirement | Recommended deployment |
|-------------|------------------------|
| Zero setup, offline | Single binary |
| Team scale, some cloud | Single server + optional remote provider |
| High availability | Multi-replica cluster |
| Strict air-gap | Air-gapped sovereign |

## See also

- [Providers and Local-First](providers-and-local-first.md)
- [Policy and Data Sovereignty](policy-and-data-sovereignty.md)
