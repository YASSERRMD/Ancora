/// Langfuse exporter: maps Ancora spans/events to Langfuse trace/observation payloads.
use crate::otlp::OtlpSpan;

#[derive(Debug, Clone)]
pub struct LangfuseConfig {
    pub host: String,
    pub public_key: String,
    pub secret_key: String,
    pub project_id: Option<String>,
}

impl LangfuseConfig {
    pub fn new(
        host: impl Into<String>,
        public_key: impl Into<String>,
        secret_key: impl Into<String>,
    ) -> Self {
        LangfuseConfig {
            host: host.into(),
            public_key: public_key.into(),
            secret_key: secret_key.into(),
            project_id: None,
        }
    }

    pub fn with_project(mut self, project_id: impl Into<String>) -> Self {
        self.project_id = Some(project_id.into());
        self
    }

    pub fn base_url(&self) -> String {
        format!("{}/api/public", self.host.trim_end_matches('/'))
    }
}

#[derive(Debug, Clone)]
pub struct LangfuseTrace {
    pub id: String,
    pub name: String,
    pub user_id: Option<String>,
    pub metadata: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
pub struct LangfuseObservation {
    pub id: String,
    pub trace_id: String,
    pub name: String,
    pub kind: ObservationKind,
    pub input: Option<String>,
    pub output: Option<String>,
    pub duration_ms: Option<f64>,
    pub metadata: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ObservationKind {
    Span,
    Generation,
    Event,
}

impl ObservationKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            ObservationKind::Span => "SPAN",
            ObservationKind::Generation => "GENERATION",
            ObservationKind::Event => "EVENT",
        }
    }
}

/// Maps an OTLP span to a Langfuse observation.
pub fn map_span_to_observation(span: &OtlpSpan) -> LangfuseObservation {
    let trace_id_hex = hex_encode(&span.trace_id);
    let span_id_hex = hex_encode(&span.span_id);
    let duration_ms = span.duration_ns() as f64 / 1_000_000.0;

    let mut metadata: Vec<(String, String)> = Vec::new();
    for (k, v) in &span.attributes {
        metadata.push((k.clone(), v.clone()));
    }

    LangfuseObservation {
        id: span_id_hex,
        trace_id: trace_id_hex,
        name: span.name.clone(),
        kind: ObservationKind::Span,
        input: None,
        output: None,
        duration_ms: if span.end_ns > span.start_ns {
            Some(duration_ms)
        } else {
            None
        },
        metadata,
    }
}

/// Extracts the Langfuse trace id from span attributes, falling back to hex trace_id.
pub fn extract_trace_id(span: &OtlpSpan) -> String {
    for (k, v) in &span.attributes {
        if k == "langfuse.trace_id" {
            return v.clone();
        }
    }
    hex_encode(&span.trace_id)
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
