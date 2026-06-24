use std::sync::Arc;

use ancora_proto::ancora::{AgentSpec, Message as ProtoMessage};

use crate::journal::JournalStore;
use crate::run::Run;

/// Single-agent runtime built from an `AgentSpec`.
pub struct Agent {
    pub spec: AgentSpec,
    pub run: Run,
    messages: Vec<ProtoMessage>,
    step: u32,
    store: Arc<dyn JournalStore>,
}

impl Agent {
    pub fn new(spec: AgentSpec, run_id: impl Into<String>, store: Arc<dyn JournalStore>) -> Self {
        Self {
            spec,
            run: Run::new(run_id),
            messages: Vec::new(),
            step: 0,
            store,
        }
    }
}
