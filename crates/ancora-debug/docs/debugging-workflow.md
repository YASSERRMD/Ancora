# Debugging Workflow

This guide walks through a typical session using ancora-debug to diagnose
a misbehaving agent run.

## 1. Load the Journal

Retrieve the raw journal entries from your persistence layer (database,
file, object store) and pass them to `DebugSession::new`:

```rust
use ancora_debug::api::DebugSession;

let entries = my_store.fetch_run("run-abc123")?;
let mut session = DebugSession::new(entries)?;
```

## 2. Review the Summary

```rust
let summary = session.summary();
println!("Run: {}", summary["run_id"]);
println!("Entries: {}", summary["entry_count"]);
```

## 3. Step Through and Annotate

Use the replayer to walk the run and mark suspicious steps:

```rust
use ancora_debug::loader::Seq;

for seq in 0..entry_count {
    let state = session.state_at(Seq(seq));
    let llm   = session.llm_at(Seq(seq));
    let tool  = session.tool_at(Seq(seq));

    if tool.map(|t| t.output.contains("error")).unwrap_or(false) {
        session.annotate_tagged(Seq(seq), "tool returned error", "bug");
    }
}
```

## 4. Diff Against a Known-Good Run

```rust
let good_entries = my_store.fetch_run("run-good-123")?;
session.load_secondary(good_entries)?;
let diff = session.diff()?;

if let Some(first_diff_seq) = diff.first_divergence {
    println!("Runs diverged at seq {}", first_diff_seq.0);
}
```

## 5. Branch and Test a Fix

```rust
// Branch just before the divergence.
session.create_branch("fix-attempt", first_diff_seq)?;

// Simulate a corrected tool response.
let corrected = JournalEntry::new(...);
session.extend_branch("fix-attempt", corrected)?;

// Inspect the new path.
let branch_j = session.branch_journal("fix-attempt", RunId::new("fix-run"))?;
let insp = Inspector::new(&branch_j);
println!("{:?}", insp.state_at(Seq(branch_j.len() - 1)));
```

## 6. Export Findings

All annotations can be retrieved for export or display in the Studio:

```rust
for ann in session.all_annotations() {
    println!("seq {}: [{}] {}", ann.seq.0, ann.tag.as_deref().unwrap_or("-"), ann.text);
}
```

## Key Properties

- Every operation is offline - no network calls are ever made.
- The original journal is never mutated.
- Branches are independent; multiple branches can coexist in one session.
