# Contributing to ancora

Thank you for contributing to ancora. This document explains how to use the
`ancora-contrib` crate to build a new extension that meets the project's
quality bar.

## Quick start

1. Pick the extension point you want to implement (provider, vector-store,
   tool, grader, guardrail, exporter, or plugin).
2. Copy the corresponding `*_template.rs` module from `ancora-contrib/src/`
   into your own crate.
3. Rename the struct and `*_id()` method to your extension's identifier.
4. Replace all `TODO` comments with real implementation code.
5. Run the built-in conformance suite (see `conformance.rs`) to verify your
   extension satisfies the SDK contract.
6. Add at least one test per public method.
7. Open a pull request against `main`.

## Development setup

```
git clone https://github.com/YASSERRMD/Ancora
cd Ancora
cargo build
cargo test -p ancora-contrib
```

## Code style

- No panics in library code: use `Result` or `Option`.
- No external dependencies unless absolutely necessary.
- Document every public item with a `///` doc comment.
- Keep functions short and single-purpose.

## Commit conventions

Use conventional commits: `feat`, `fix`, `docs`, `test`, `refactor`, `chore`.

## Questions

Open an issue with the label `question` if you are unsure how to proceed.
