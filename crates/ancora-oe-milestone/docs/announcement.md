# Announcement: Ancora Observability and Eval Milestone

**Date:** 2026-06-29
**Channels:** blog, discord, release-notes

## Summary

The Ancora observability and eval milestone is now complete. Every suite is
green, cross-language parity is achieved, documentation is consolidated, and
the metrics and evals catalog is published.

## What's New

- Full GA for distributed tracing, metrics export, and eval gating across
  Rust, Python, TypeScript (Go at Beta for some features)
- `ancora-evallib`: offline eval computation - no network required
- `ancora-obssdk`: unified SDK wrapper for all obs signals
- Self-hosted LGTM stack Helm chart (HA and federated topologies)
- Privacy label scrubbing enabled by default
- 30-day default metrics retention (up from 15 days)

## Key Numbers

- 10 obs and eval crates fully green
- 21 commits in the Phase 240 milestone
- 5 language quickstart guides published
- 0 open P0 issues

## Upgrade

See [upgrade notes](upgrade_notes.md) for the single breaking config rename.
All other changes are non-breaking.

## Roadmap

Next milestone: Ecosystem and Edge (Phase 250)
- WebAssembly eval runner
- Edge-native metric aggregation
- Multi-model eval harness
