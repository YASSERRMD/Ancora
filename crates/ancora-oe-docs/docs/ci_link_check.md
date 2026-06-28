# CI: Observability and Eval Docs Build and Link Check

This document describes the CI workflow that builds and link-checks the OE docs.

## Steps

1. Build the crate: `cargo build -p ancora-oe-docs`
2. Run tests: `cargo test -p ancora-oe-docs`
3. Check all internal doc links are valid
4. Verify all checklist item IDs are unique
