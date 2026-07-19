# Contributing to Ancora

Thank you for your interest in contributing to Ancora. This guide explains
how to set up a development environment, the standards your changes must
meet, and how to get them merged. Contributions of all kinds are welcome:
bug reports, fixes, features, documentation, and SDK bindings.

## Getting started

1. Fork the repository and clone your fork.
2. Install the Rust toolchain. The required version is pinned in
   [`rust-toolchain.toml`](rust-toolchain.toml) and is picked up
   automatically by `rustup`.
3. Build and test the workspace:

```bash
cargo build --all
cargo test --all
```

The full test suite runs offline. If a test you add requires network
access, it will not be accepted; see [Testing](#testing) below.

Before committing, make sure git is configured with your real name and a
valid email address so your work is correctly attributed:

```bash
git config user.name "Your Name"
git config user.email "you@example.com"
```

## Reporting issues

Open a GitHub issue with:

- What you expected to happen and what happened instead.
- Steps to reproduce, ideally as a minimal example.
- Your platform, Rust version, and the Ancora version or commit hash.

For security vulnerabilities, please do not open a public issue. Refer to
the [threat model](docs/security/threat-model.md) and report privately to
the maintainers.

## Development workflow

`main` is protected. All changes land through a pull request that has
passed CI; direct pushes are not possible.

1. Create a branch from an up-to-date `main`:

```bash
git checkout main
git pull origin main
git checkout -b feat/short-description
```

   Use a `type/short-slug` branch name that matches the change, for
   example `fix/journal-replay-offset` or `docs/quickstart-go`.

2. Make your changes in atomic commits: one logical change per commit,
   with its tests in the same commit or the one immediately following.

3. Push the branch to your fork and open a pull request against `main`.

## Commit conventions

Commits follow [Conventional Commits](https://www.conventionalcommits.org/)
and are enforced by the `commit-lint` CI check:

```
type(scope): imperative summary under 72 chars

Why the change is needed and what problem it solves. Wrap the body at
72 columns.
```

- Allowed types: `feat`, `fix`, `refactor`, `test`, `docs`, `chore`,
  `build`, `ci`, `perf`, `style`.
- The scope is the affected crate or package, for example `core`,
  `proto`, `ffi`, or `go-sdk`.
- Em dash characters are not permitted anywhere in the repository,
  including commit messages. Use a plain hyphen instead.

## Continuous integration

Every pull request must pass the following checks before it can merge:

| Check | Command |
|-------|---------|
| fmt | `cargo fmt --all -- --check` |
| clippy | `cargo clippy --all -- -D warnings` |
| build | `cargo build --all` |
| test | `cargo test --all` |
| commit-lint | Conventional Commits and style rules |

Running these locally before pushing saves a review round trip.

## Testing

- Tests ship with the code they cover, in the same or immediately
  following commit.
- The full suite must pass offline. The default model adapter targets a
  local OpenAI-compatible endpoint, so no test may depend on external
  network access or cloud credentials.
- Non-deterministic activities (model calls, tool calls) must be
  journaled on first execution and never re-executed on replay. Changes
  that break replay determinism will not be accepted.

## Pull requests

A good pull request:

- Does one thing, described clearly in the title and body.
- Explains the motivation, the approach, and any trade-offs.
- Lists the tests added or updated.
- Keeps its commit history clean and atomic.

Maintainers merge approved pull requests with a merge commit to preserve
the commit history; pull requests are not squashed. The source branch is
deleted after merge. If you need to follow up on an already merged change,
open a new branch and a new pull request rather than reusing the old
branch.

## Documentation

Documentation lives in [`docs/`](docs/) and is built with MkDocs. Writing
conventions, link rules, and validation scripts are described in
[docs/contributing.md](docs/contributing.md).

## License

Ancora is licensed under the [Apache License 2.0](LICENSE). By submitting
a contribution you agree that it is licensed under the same terms.
