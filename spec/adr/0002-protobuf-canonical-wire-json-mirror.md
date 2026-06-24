# ADR-0002: Protobuf as canonical wire format with JSON mirror

Date: 2026-06-24
Status: Accepted

## Context

Ancora passes messages between the core engine, language bindings, and
the gRPC sidecar. The format must be: compact for high-frequency journal
writes, schema-validated to prevent silent corruption, evolvable without
breaking existing readers, and consumable by languages that cannot easily
link protobuf runtime libraries.

## Decision

Protobuf (proto3) is the canonical wire format for all Ancora messages
and the event journal. Every type in ancora-proto also has a serde-JSON
serialization and a generated JSON Schema so that non-proto consumers can
validate and process messages without a protobuf runtime.

## Consequences

- Protobuf gives compact binary encoding, deterministic field ordering
  within a message, and forward/backward compatibility via field
  numbers.
- prost generates idiomatic Rust types from .proto files; tonic-build
  wires the gRPC service definitions.
- The JSON mirror means CLI tools, test fixtures, and language bindings
  that prefer text formats can participate without a .proto toolchain.
- JSON Schema is generated at build time and committed to spec/schema so
  it is available without a build step.
- Maintaining two representations requires a round-trip test to catch
  divergence, added as a CI check.

## Alternatives considered

- JSON only: human-readable but larger and slower to parse for the
  journal hot path.
- MessagePack: compact but no schema enforcement and weaker ecosystem
  support across all target languages.
- Cap'n Proto / FlatBuffers: excellent performance but smaller language
  binding ecosystems and more complex schema evolution rules.
