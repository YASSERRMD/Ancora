# Contributing to Ancora

## Git identity

Every commit must be authored as **YASSERRMD** with email
`arafath.yasser@gmail.com`. Before your first push on any branch run:

```bash
git config user.name "YASSERRMD"
git config user.email "arafath.yasser@gmail.com"
```

Verify before every push:

```bash
git config user.name   # must print: YASSERRMD
git config user.email  # must print: arafath.yasser@gmail.com
```

## Branch protection

`main` is protected. You cannot push to it directly. All changes reach
`main` only through a merged pull request that has passed CI.

Required CI checks before merge:
- fmt (rustfmt --check)
- clippy (deny warnings)
- build (cargo build --all)
- test (cargo test --all)
- commit-lint (Conventional Commits + no em dash)

## Git workflow

### Starting a phase

```bash
git checkout main
git pull origin main
git checkout -b phase-NN-short-slug
```

Use zero-padded two-digit numbers, for example `phase-03-event-journal`.

### Committing

Commit each atomic task separately immediately after it is complete and
its tests pass. Follow Conventional Commits:

```
type(scope): imperative summary under 72 chars

Why this change is needed and what problem it solves. Wrap at 72
columns. Reference the phase and atomic task. No em dash characters.

Phase: NN
```

Allowed types: `feat`, `fix`, `refactor`, `test`, `docs`, `chore`,
`build`, `ci`, `perf`, `style`.

Scope is the crate or package name: `core`, `proto`, `ffi`, `go-sdk`,
etc.

### Finishing a phase

Push the branch once the phase is functionally complete and all tests
pass:

```bash
git push -u origin phase-NN-short-slug
```

Open a PR into `main`:

```bash
gh pr create --base main --head phase-NN-short-slug \
  --title "Phase NN: <title>" \
  --body "<summary, tests added, decisions>"
```

After CI is green, squash-merge and delete the branch:

```bash
gh pr merge --squash --delete-branch
git checkout main && git pull origin main
git branch -d phase-NN-short-slug
```

### Fixes to merged phases

Never reopen an old phase branch. Create a new branch:

```bash
git checkout -b fix-NN-short-slug
```

Commit the fix atomically, push, open a PR, merge, delete.

## Standing rules

1. No em dash anywhere: not in code, comments, docs, or commits.
2. Atomic commits: one logical change per commit.
3. No direct work on `main`.
4. Tests ship with code in the same or immediately following commit.
5. Default model adapter targets a local OpenAI-compatible endpoint.
   The full test suite must pass offline.
6. Non-deterministic activities (model calls, tool calls) must be
   journaled on first execution and never re-run on replay.
