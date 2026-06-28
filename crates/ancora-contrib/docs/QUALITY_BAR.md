# Quality bar and checklist

All contributions to ancora must meet the following quality bar before they
are accepted.

## Non-negotiable requirements

| Requirement | How to verify |
|-------------|---------------|
| Builds without errors | `cargo build -p <crate>` |
| All tests pass | `cargo test -p <crate>` |
| No panics in library code | Code review; prefer `Result`/`Option` |
| Implements the correct SDK trait | Code review; check trait bounds |
| Conformance suite passes | Run `conformance::*_suite().run()` |
| No network calls in tests | Code review; tests must be offline |

## Recommended

| Recommendation | Rationale |
|----------------|-----------|
| Zero clippy warnings | `cargo clippy -- -D warnings` |
| Formatted with rustfmt | `cargo fmt --check` |
| Every public item documented | `cargo doc --no-deps` |
| Error types implement `Display` + `Error` | Ergonomic error handling |
| Struct fields have `pub` visibility | Testability |

## Contribution checklist

Copy this checklist into your PR description:

```
- [ ] cargo build passes with no errors
- [ ] cargo test passes with no failures
- [ ] conformance suite passes (report.all_passed() == true)
- [ ] no panics in library code
- [ ] at least one test per public method
- [ ] docs/README.md present
- [ ] cargo fmt applied
- [ ] cargo clippy clean
```

## Grading rubric for reviewers

- 5 (merge): all requirements met, excellent test coverage, clear docs.
- 4 (minor changes): all requirements met, one or two small improvements requested.
- 3 (major changes): missing tests, docs, or conformance suite not run.
- 2 (redesign): trait not correctly implemented or fundamental design issue.
- 1 (reject): panics in library code, network calls in tests, or build fails.
