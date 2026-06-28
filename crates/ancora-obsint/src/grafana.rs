/// Grafana backend exporter: maps spans to Tempo and log records to Loki.

use crate::otlp::{OtlpMetricPoint, OtlpSpan};

#[derive(Debug, Clone)]
pub struct GrafanaConfig {
    pub tempo_endpoint: String,
    pub loki_endpoint: String,
    pub auth_token: Option<String>,
    pub org_id: Option<String>,
}

impl GrafanaConfig {
    pub fn new(
        tempo_endpoint: impl Into<String>,
        loki_endpoint: impl Into<String>,
    ) -> Self {
        GrafanaConfig {
            tempo_endpoint: tempo_endpoint.into(),
            loki_endpoint: loki_endpoint.into(),
            auth_token: None,
            org_id: None,
        }
    }

    pub fn with_auth(mut self, token: impl Into<String>) -> Self {
        self.auth_token = Some(token.into());
        self
    }

    pub fn with_org_id(mut self, org_id: impl Into<String>) -> Self {
        self.org_id = Some(org_id.into());
        self
    }
}

/// A Tempo-compatible span representation.
#[derive(Debug, Clone)]
pub struct TempoSpan {
    pub trace_id: String,
    pub span_id: String,
    pub operation_name: String,
    pub start_time_us: u64,
    pub duration_us: u64,
    pub tags: Vec<(String, String)>,
}

/// Maps an OTLP span to a Tempo span.
pub fn map_span_to_tempo(span: &OtlpSpan) -> TempoSpan {
    TempoSpan {
        trace_id: hex_encode(&span.trace_id),
        span_id: hex_encode(&span.span_id),
        operation_name: span.name.clone(),
        start_time_us: span.start_ns / 1000,
        duration_us: span.duration_ns() / 1000,
        tags: span.attributes.clone(),
    }
}

/// A Loki log stream label set.
#[derive(Debug, Clone)]
pub struct LokiLabels {
    pub labels: Vec<(String, String)>,
}

/// A single Loki log entry.
#[derive(Debug, Clone)]
pub struct LokiEntry {
    pub timestamp_ns: u64,
    pub line: String,
}

/// A Loki stream (labels + entries).
#[derive(Debug, Clone)]
pub struct LokiStream {
    pub labels: LokiLabels,
    pub entries: Vec<LokiEntry>,
}

impl LokiStream {
    pub fn new(labels: Vec<(String, String)>) -> Self {
        LokiStream {
            labels: LokiLabels { labels },
            entries: Vec::new(),
        }
    }

    pub fn push_entry(&mut self, timestamp_ns: u64, line: impl Into<String>) {
        self.entries.push(LokiEntry {
            timestamp_ns,
            line: line.into(),
        });
    }
}

/// Maps a metric point to a Loki log line for debugging/auditing.
pub fn metric_to_loki_line(point: &OtlpMetricPoint) -> String {
    let label_str: Vec<String> = point
        .labels
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect();
    format!(
        "metric={} value={} labels=[{}] ts={}",
        point.name,
        point.value,
        label_str.join(","),
        point.timestamp_ns
    )
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
