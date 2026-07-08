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
    tracing::info_span!("ancora.run", run.id = run_id, run.agent = agent_name,)
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

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::field::Visit;
    /// Collect span field names into a Vec for assertions.
    struct FieldCollector(Vec<String>);

    impl Visit for FieldCollector {
        fn record_str(&mut self, field: &tracing::field::Field, _value: &str) {
            self.0.push(field.name().to_string());
        }
        fn record_u64(&mut self, field: &tracing::field::Field, _value: u64) {
            self.0.push(field.name().to_string());
        }
        fn record_debug(&mut self, field: &tracing::field::Field, _value: &dyn std::fmt::Debug) {
            self.0.push(field.name().to_string());
        }
    }

    fn field_names(span: &Span) -> Vec<String> {
        let mut collector = FieldCollector(Vec::new());
        if let Some(meta) = span.metadata() {
            for field in meta.fields() {
                collector.0.push(field.name().to_string());
            }
        }
        collector.0
    }

    fn setup_subscriber() {
        let _ = tracing_subscriber::fmt().with_test_writer().try_init();
    }

    #[test]
    fn run_span_carries_required_fields() {
        setup_subscriber();
        let span = run_span("run-abc", "my-agent");
        let fields = field_names(&span);
        assert!(
            fields.contains(&field::RUN_ID.to_string()),
            "missing run.id"
        );
        assert!(
            fields.contains(&field::RUN_AGENT.to_string()),
            "missing run.agent"
        );
    }

    #[test]
    fn node_span_carries_required_fields() {
        setup_subscriber();
        let span = node_span("run-abc", "plan", 3);
        let fields = field_names(&span);
        assert!(
            fields.contains(&field::RUN_ID.to_string()),
            "missing run.id"
        );
        assert!(
            fields.contains(&field::NODE_NAME.to_string()),
            "missing node.name"
        );
        assert!(
            fields.contains(&field::NODE_SEQ.to_string()),
            "missing node.seq"
        );
    }

    #[test]
    fn activity_span_carries_required_fields() {
        setup_subscriber();
        let span = activity_span("run-abc", "model_call", 5);
        let fields = field_names(&span);
        assert!(
            fields.contains(&field::RUN_ID.to_string()),
            "missing run.id"
        );
        assert!(
            fields.contains(&field::ACTIVITY_KIND.to_string()),
            "missing activity.kind"
        );
        assert!(
            fields.contains(&field::ACTIVITY_SEQ.to_string()),
            "missing activity.seq"
        );
    }

    #[test]
    fn span_names_are_stable() {
        setup_subscriber();
        assert_eq!(
            run_span("x", "y").metadata().map(|m| m.name()),
            Some("ancora.run")
        );
        assert_eq!(
            node_span("x", "y", 0).metadata().map(|m| m.name()),
            Some("ancora.node")
        );
        assert_eq!(
            activity_span("x", "y", 0).metadata().map(|m| m.name()),
            Some("ancora.activity")
        );
    }
}
