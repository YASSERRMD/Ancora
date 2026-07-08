#![deny(clippy::all)]

use std::collections::{HashMap, VecDeque};

use napi::bindgen_prelude::*;
use napi_derive::napi;

struct InnerRun {
    id: String,
    events: VecDeque<String>,
}

impl InnerRun {
    fn new(id: &str, spec: &str) -> Self {
        let escaped = spec.replace('\\', "\\\\").replace('"', "\\\"");
        let mut events = VecDeque::new();
        events.push_back(format!(
            r#"{{"kind":"started","run_id":"{}","spec":"{}"}}"#,
            id, escaped
        ));
        for token in &["Hello", " ", "world"] {
            events.push_back(format!(
                r#"{{"kind":"token","run_id":"{}","text":"{}"}}"#,
                id, token
            ));
        }
        events.push_back(format!(r#"{{"kind":"completed","run_id":"{}"}}"#, id));
        Self {
            id: id.to_string(),
            events,
        }
    }

    fn poll(&mut self) -> Option<String> {
        self.events.pop_front()
    }

    fn resume(&mut self, decision: &str) {
        self.events.push_back(format!(
            r#"{{"kind":"resumed","run_id":"{}","decision":"{}"}}"#,
            self.id, decision
        ));
        self.events
            .push_back(format!(r#"{{"kind":"completed","run_id":"{}"}}"#, self.id));
    }
}

/// Handle to the Ancora agent runtime (napi-rs Node.js binding).
#[napi]
pub struct Runtime {
    runs: Option<HashMap<String, InnerRun>>,
}

#[napi]
impl Runtime {
    /// Create a new Runtime instance.
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            runs: Some(HashMap::new()),
        }
    }

    /// Return true if the runtime has been freed.
    #[napi(getter)]
    pub fn is_freed(&self) -> bool {
        self.runs.is_none()
    }

    /// Release the runtime. Subsequent calls are no-ops.
    #[napi]
    pub fn free(&mut self) {
        self.runs = None;
    }

    /// Start a new agent run from spec bytes. Returns the run ID.
    #[napi]
    pub fn start_run(&mut self, spec_bytes: Buffer) -> Result<String> {
        let runs = self
            .runs
            .as_mut()
            .ok_or_else(|| Error::new(Status::GenericFailure, "Runtime has been freed"))?;
        let spec = String::from_utf8_lossy(&spec_bytes).into_owned();
        let run_id = uuid::Uuid::new_v4().to_string();
        runs.insert(run_id.clone(), InnerRun::new(&run_id, &spec));
        Ok(run_id)
    }

    /// Poll the next event for a run. Returns null when exhausted.
    #[napi]
    pub fn poll_run(&mut self, run_id: String) -> Result<Option<Buffer>> {
        let runs = self
            .runs
            .as_mut()
            .ok_or_else(|| Error::new(Status::GenericFailure, "Runtime has been freed"))?;
        let event = runs.get_mut(&run_id).and_then(|r| r.poll());
        Ok(event.map(|s| Buffer::from(s.into_bytes())))
    }

    /// Resume a suspended run with a decision payload.
    #[napi]
    pub fn resume_run(&mut self, run_id: String, decision: Buffer) -> Result<()> {
        let runs = self
            .runs
            .as_mut()
            .ok_or_else(|| Error::new(Status::GenericFailure, "Runtime has been freed"))?;
        let decision_str = String::from_utf8_lossy(&decision).into_owned();
        if let Some(run) = runs.get_mut(&run_id) {
            run.resume(&decision_str);
        }
        Ok(())
    }
}

/// Return the SDK version string.
#[napi]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
