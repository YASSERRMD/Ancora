# Trace Replay Debugging

ancora-debug provides offline trace replay debugging for Ancora agent runs.

## Overview

Every Ancora run produces a journal - a sequence of entries recording state
transitions, LLM exchanges, and tool calls. The ancora-debug crate loads
that journal and lets you step through it, inspect every detail, and diff
against other runs, all without making any live calls.

## Loading a Journal

```rust
use ancora_debug::loader::{load_journal, JournalEntry, RunId, EntryKind};

let entries: Vec<JournalEntry> = /* load from storage */;
let journal = load_journal(entries)?;
```

The loader validates that sequence numbers are contiguous and all entries
belong to the same run. It returns `LoadError` on malformed input.

## Step-Through Replay

```rust
use ancora_debug::replay::{Replayer, StepResult};

let mut r = Replayer::new(&journal);
loop {
    match r.step_forward() {
        StepResult::Stepped(entry) => println!("seq {}: {:?}", entry.seq.0, entry.kind),
        StepResult::AtBoundary => break,
    }
}
```

You can also step backward, seek to any sequence number, and reset to the
beginning.

## Inspecting State

```rust
use ancora_debug::inspector::Inspector;
use ancora_debug::loader::Seq;

let insp = Inspector::new(&journal);
let state = insp.state_at(Seq(10)).unwrap();
let llm   = insp.llm_at(Seq(10)).unwrap();
let tool  = insp.tool_at(Seq(10)).unwrap();
```

Each method walks backward from the given sequence number to find the most
recent entry of that kind.

## High-Level API

The `DebugSession` type bundles all functionality into a single interface:

```rust
use ancora_debug::api::DebugSession;
use ancora_debug::loader::Seq;

let mut session = DebugSession::new(entries)?;
println!("{:?}", session.state_at(Seq(5)));
session.annotate(Seq(5), "worth investigating");
```
