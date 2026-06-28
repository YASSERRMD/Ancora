/// Datadog exporter: maps spans and metrics to Datadog APM and metrics API payloads.

use crate::otlp::{OtlpMetricPoint, OtlpSpan};

#[derive(Debug, Clone)]
pub struct DatadogConfig {
    pub site: String,
    pub api_key: String,
    pub service: String,
    pub env: String,
    pub version: Option<String>,
}

impl DatadogConfig {
    pub fn new(
        api_key: impl Into<String>,
        service: impl Into<String>,
        env: impl Into<String>,
    ) -> Self {
        DatadogConfig {
            site: "datadoghq.com".to_string(),
            api_key: api_key.into(),
            service: service.into(),
            env: env.into(),
            version: None,
        }
    }

    pub fn with_site(mut self, site: impl Into<String>) -> Self {
        self.site = site.into();
        self
    }

    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    pub fn trace_endpoint(&self) -> String {
        format!("https://trace.agent.{}/api/v0.2/traces", self.site)
    }

    pub fn metrics_endpoint(&self) -> String {
        format!("https://api.{}/api/v1/series", self.site)
    }
}

#[derive(Debug, Clone)]
pub struct DatadogSpan {
    pub trace_id: u64,
    pub span_id: u64,
    pub name: String,
    pub resource: String,
    pub service: String,
    pub span_type: String,
    pub start_ns: i64,
    pub duration_ns: i64,
    pub error: i32,
    pub meta: Vec<(String, String)>,
}

/// Maps an OTLP span to a Datadog APM span.
/// Datadog uses 64-bit unsigned integers for trace/span IDs.
pub fn map_span_to_datadog(span: &OtlpSpan, cfg: &DatadogConfig) -> DatadogSpan {
    let trace_id = u64::from_le_bytes(span.trace_id[..8].try_into().unwrap_or([0u8; 8]));
    let span_id = u64::from_le_bytes(span.span_id);

    let resource = span
        .attributes
        .iter()
        .find(|(k, _)| k == "http.route" || k == "db.statement" || k == "rpc.method")
        .map(|(_, v)| v.clone())
        .unwrap_or_else(|| span.name.clone());

    let span_type = span
        .attributes
        .iter()
        .find(|(k, _)| k == "span.type")
        .map(|(_, v)| v.clone())
        .unwrap_or_else(|| "custom".to_string());

    DatadogSpan {
        trace_id,
        span_id,
        name: span.name.clone(),
        resource,
        service: cfg.service.clone(),
        span_type,
        start_ns: span.start_ns as i64,
        duration_ns: span.duration_ns() as i64,
        error: if span.status_code == 2 { 1 } else { 0 },
        meta: span.attributes.clone(),
    }
}

#[derive(Debug, Clone)]
pub struct DatadogMetric {
    pub metric: String,
    pub points: Vec<(i64, f64)>,
    pub metric_type: String,
    pub tags: Vec<String>,
    pub host: Option<String>,
}

/// Maps an OTLP metric point to a Datadog series entry.
pub fn map_metric_to_datadog(point: &OtlpMetricPoint, cfg: &DatadogConfig) -> DatadogMetric {
    let ts = (point.timestamp_ns / 1_000_000_000) as i64;
    let tags: Vec<String> = point
        .labels
        .iter()
        .map(|(k, v)| format!("{}:{}", k, v))
        .chain(std::iter::once(format!("env:{}", cfg.env)))
        .chain(std::iter::once(format!("service:{}", cfg.service)))
        .collect();

    DatadogMetric {
        metric: point.name.clone(),
        points: vec![(ts, point.value)],
        metric_type: "gauge".to_string(),
        tags,
        host: None,
    }
}
