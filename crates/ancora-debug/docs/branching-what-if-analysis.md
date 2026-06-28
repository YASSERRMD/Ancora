# Branching and What-If Analysis

ancora-debug lets you branch from any point in a recorded run to explore
alternative continuations without modifying the original journal.

## Creating a Branch

```rust
use ancora_debug::api::DebugSession;
use ancora_debug::loader::{EntryKind, JournalEntry, RunId, Seq};

let mut session = DebugSession::new(original_entries)?;

// Branch at seq 3 - keep entries 0..=3 and add new ones.
session.create_branch("what-if-faster-path", Seq(3))?;
```

## Extending a Branch

After creating a branch you can append new entries representing the
alternative continuation:

```rust
let new_entry = JournalEntry::new(
    RunId::new("branch-run"),
    0, // seq is re-assigned automatically
    EntryKind::StateChange { from: "planning".into(), to: "fast-done".into() },
);
session.extend_branch("what-if-faster-path", new_entry)?;
```

## Materialising a Branch as a Journal

```rust
let branch_journal = session.branch_journal(
    "what-if-faster-path",
    RunId::new("branch-run-id"),
)?;
// branch_journal can be used with Inspector, Replayer, etc.
```

## Diffing the Branch Against the Original

Load the branch journal as a secondary and diff:

```rust
session.load_secondary(branch_journal_entries)?;
let diff = session.diff()?;
println!("First divergence at: {:?}", diff.first_divergence);
```

## Use Cases

- Explore what would have happened with a different tool output.
- Test alternative state machine paths without re-running the agent.
- Identify the minimal change that fixes an observed bug.
