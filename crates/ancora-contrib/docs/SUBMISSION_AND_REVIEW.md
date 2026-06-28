# Submission and review process

## Before you open a PR

- [ ] Run `cargo build -p <your-crate>` with no warnings.
- [ ] Run `cargo test -p <your-crate>` with all tests passing.
- [ ] Run the built-in conformance suite for your extension kind and confirm
      `report.all_passed()` is `true`.
- [ ] Add at least one test per public method.
- [ ] Provide a `docs/README.md` that explains usage and configuration.
- [ ] Format your code with `cargo fmt`.
- [ ] Check for lints with `cargo clippy -- -D warnings`.

## Opening the PR

- Target branch: `main`.
- PR title: `feat(<crate>): <short description>` (50 chars max).
- PR body: describe what the extension does and how to use it.
- Link to any upstream service documentation.

## Review criteria

1. Does the extension implement the correct trait from the plugin SDK?
2. Do all unit tests pass offline (no network calls in tests)?
3. Does the conformance suite pass?
4. Is there a docs stub?
5. Is the code free of panics in library code?

## After review

- Address all review comments in new commits (do not force-push).
- Once approved, a maintainer will merge with a merge commit (no squash).
- Your extension will appear in the plugin catalog after the next release.
