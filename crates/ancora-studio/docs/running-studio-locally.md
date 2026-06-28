# Running the Studio Locally

Ancora Studio is a library crate. It ships as a Rust library with an in-memory backend that works fully offline. Integration into a TUI or web frontend is left to the application layer.

## Prerequisites

- Rust 1.70+ (stable toolchain)
- No network access required

## Building

```bash
cargo build -p ancora-studio
```

## Running Tests

```bash
cargo test -p ancora-studio
```

All tests run offline against an in-memory backend. No external services are needed.

## Using the Demo Backend

The `backend::demo_backend()` function returns a pre-populated `InMemoryBackend` suitable for exploring the API without any real data:

```rust
use ancora_studio::backend::{demo_backend, StudioBackend};

fn main() {
    let backend = demo_backend();
    let runs = backend.list_runs().unwrap();
    for run in runs.all() {
        println!("{} - {}", run.id, run.label);
    }
}
```

## Wiring Up Your Own Backend

Implement the `StudioBackend` trait for any storage layer (SQLite, flat files, etc.):

```rust
use ancora_studio::backend::{StudioBackend, BackendError};
use ancora_studio::run_list::RunList;
use ancora_studio::timeline::Timeline;
use ancora_studio::eval_view::EvalView;
use ancora_studio::cost_view::CostBreakdown;
use ancora_studio::feedback_view::FeedbackView;

struct MyBackend;

impl StudioBackend for MyBackend {
    fn list_runs(&self) -> Result<RunList, BackendError> { todo!() }
    fn get_timeline(&self, run_id: &str) -> Result<Timeline, BackendError> { todo!() }
    fn get_evals(&self, run_id: &str) -> Result<EvalView, BackendError> { todo!() }
    fn get_cost_breakdown(&self, run_id: &str) -> Result<CostBreakdown, BackendError> { todo!() }
    fn get_feedback(&self, run_id: &str) -> Result<FeedbackView, BackendError> { todo!() }
}
```

## Configuring Redaction

Build a `RedactionEngine` with your field rules and pass a `ViewerContext` per request:

```rust
use ancora_studio::redaction::{RedactionEngine, FieldRedactionRule, RedactionPolicy, ViewerContext};

let engine = RedactionEngine::new(vec![
    FieldRedactionRule::new("prompt", RedactionPolicy::AlwaysRedact),
    FieldRedactionRule::new("response", RedactionPolicy::RequireRole("admin".into())),
]);

let viewer = ViewerContext::new(vec!["user".into()]);
let displayed = engine.apply("prompt", "actual prompt text", &viewer);
// displayed == "[REDACTED]"
```

## Offline Guarantee

The `InMemoryBackend` and all module types in this crate use only `std`. There are no network calls, no file I/O (unless you add them), and no external runtime dependencies. All tests pass without any network access.
