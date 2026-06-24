use tracing::Span;

/// Canonical tracing field names used across all engine spans.
pub mod field {
    pub const RUN_ID: &str = "run.id";
    pub const RUN_AGENT: &str = "run.agent";
    pub const NODE_NAME: &str = "node.name";
    pub const NODE_SEQ: &str = "node.seq";
    pub const ACTIVITY_KIND: &str = "activity.kind";
    pub const ACTIVITY_SEQ: &str = "activity.seq";
}

/// Opens a span for the lifetime of a run.
///
/// Fields: `run.id`, `run.agent`.
pub fn run_span(run_id: &str, agent_name: &str) -> Span {
    tracing::info_span!(
        "ancora.run",
        run.id = run_id,
        run.agent = agent_name,
    )
}

/// Opens a span for a single graph node execution within a run.
///
/// Fields: `run.id`, `node.name`, `node.seq`.
pub fn node_span(run_id: &str, node_name: &str, seq: u64) -> Span {
    tracing::info_span!(
        "ancora.node",
        run.id = run_id,
        node.name = node_name,
        node.seq = seq,
    )
}

/// Opens a span for a single journaled activity (model call or tool call).
///
/// Fields: `run.id`, `activity.kind`, `activity.seq`.
pub fn activity_span(run_id: &str, kind: &str, seq: u64) -> Span {
    tracing::info_span!(
        "ancora.activity",
        run.id = run_id,
        activity.kind = kind,
        activity.seq = seq,
    )
}
