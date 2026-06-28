/// Phoenix (Arize) exporter: maps spans/traces to Phoenix's OpenInference schema.

use crate::otlp::OtlpSpan;

#[derive(Debug, Clone)]
pub struct PhoenixConfig {
    pub endpoint: String,
    pub api_key: Option<String>,
    pub project_name: String,
}

impl PhoenixConfig {
    pub fn new(endpoint: impl Into<String>, project_name: impl Into<String>) -> Self {
        PhoenixConfig {
            endpoint: endpoint.into(),
            api_key: None,
            project_name: project_name.into(),
        }
    }

    pub fn with_api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SpanKind {
    Chain,
    Llm,
    Retriever,
    Embedding,
    Tool,
    Unknown,
}

impl SpanKind {
    pub fn from_str(s: &str) -> SpanKind {
        match s {
            "chain" | "CHAIN" => SpanKind::Chain,
            "llm" | "LLM" => SpanKind::Llm,
            "retriever" | "RETRIEVER" => SpanKind::Retriever,
            "embedding" | "EMBEDDING" => SpanKind::Embedding,
            "tool" | "TOOL" => SpanKind::Tool,
            _ => SpanKind::Unknown,
        }
    }

    pub fn openinference_label(&self) -> &'static str {
        match self {
            SpanKind::Chain => "CHAIN",
            SpanKind::Llm => "LLM",
            SpanKind::Retriever => "RETRIEVER",
            SpanKind::Embedding => "EMBEDDING",
            SpanKind::Tool => "TOOL",
            SpanKind::Unknown => "UNKNOWN",
        }
    }
}

#[derive(Debug, Clone)]
pub struct PhoenixSpan {
    pub span_id: String,
    pub trace_id: String,
    pub name: String,
    pub kind: SpanKind,
    pub start_time_ns: u64,
    pub end_time_ns: u64,
    pub attributes: Vec<(String, String)>,
    pub status_message: Option<String>,
}

/// Maps an OTLP span to a Phoenix/OpenInference span.
pub fn map_span_to_phoenix(span: &OtlpSpan) -> PhoenixSpan {
    let kind = span
        .attributes
        .iter()
        .find(|(k, _)| k == "openinference.span.kind")
        .map(|(_, v)| SpanKind::from_str(v))
        .unwrap_or(SpanKind::Unknown);

    let status_message = if span.status_code != 0 {
        Some(format!("status_code={}", span.status_code))
    } else {
        None
    };

    PhoenixSpan {
        span_id: hex_encode(&span.span_id),
        trace_id: hex_encode(&span.trace_id),
        name: span.name.clone(),
        kind,
        start_time_ns: span.start_ns,
        end_time_ns: span.end_ns,
        attributes: span.attributes.clone(),
        status_message,
    }
}

/// Validates Phoenix config, returns error if endpoint is empty.
pub fn validate_config(cfg: &PhoenixConfig) -> Result<(), String> {
    if cfg.endpoint.is_empty() {
        return Err("phoenix endpoint must not be empty".to_string());
    }
    if cfg.project_name.is_empty() {
        return Err("phoenix project_name must not be empty".to_string());
    }
    Ok(())
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
