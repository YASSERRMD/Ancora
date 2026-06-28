//! Exporter parity - validates that trace/metric exporters emit compatible payloads across SDKs.

use std::collections::HashMap;

/// Supported export formats.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExportFormat {
    Otlp,
    Jaeger,
    Prometheus,
    JsonLines,
}

impl ExportFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            ExportFormat::Otlp => "otlp",
            ExportFormat::Jaeger => "jaeger",
            ExportFormat::Prometheus => "prometheus",
            ExportFormat::JsonLines => "jsonlines",
        }
    }
}

/// An exported payload record.
#[derive(Debug, Clone)]
pub struct ExportedPayload {
    pub language: String,
    pub format: ExportFormat,
    pub span_count: usize,
    pub metric_count: usize,
    pub headers: HashMap<String, String>,
    pub body_bytes: usize,
}

impl ExportedPayload {
    pub fn new(language: impl Into<String>, format: ExportFormat) -> Self {
        Self {
            language: language.into(),
            format,
            span_count: 0,
            metric_count: 0,
            headers: HashMap::new(),
            body_bytes: 0,
        }
    }

    pub fn with_spans(mut self, count: usize) -> Self {
        self.span_count = count;
        self
    }

    pub fn with_metrics(mut self, count: usize) -> Self {
        self.metric_count = count;
        self
    }

    pub fn with_body_bytes(mut self, bytes: usize) -> Self {
        self.body_bytes = bytes;
        self
    }

    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }
}

/// Simulate exporting a payload for a given language.
pub fn simulate_export(language: impl Into<String>, format: ExportFormat) -> ExportedPayload {
    let lang = language.into();
    ExportedPayload::new(&lang, format)
        .with_spans(2)
        .with_metrics(3)
        .with_body_bytes(1024)
        .with_header("Content-Type", "application/x-protobuf")
        .with_header("X-Language", &lang)
}

/// Check that all exported payloads are structurally equivalent.
pub fn check_exporter_parity(payloads: &[ExportedPayload]) -> Vec<String> {
    let mut issues = Vec::new();
    if let Some(first) = payloads.first() {
        for other in payloads.iter().skip(1) {
            if first.format != other.format {
                issues.push(format!(
                    "format mismatch: {:?}={:?} vs {:?}={:?}",
                    first.language, first.format.as_str(),
                    other.language, other.format.as_str()
                ));
            }
            if first.span_count != other.span_count {
                issues.push(format!(
                    "span_count mismatch: {:?}={} vs {:?}={}",
                    first.language, first.span_count,
                    other.language, other.span_count
                ));
            }
            if first.metric_count != other.metric_count {
                issues.push(format!(
                    "metric_count mismatch: {:?}={} vs {:?}={}",
                    first.language, first.metric_count,
                    other.language, other.metric_count
                ));
            }
            // Check required headers are present.
            for key in &["Content-Type"] {
                if !other.headers.contains_key(*key) {
                    issues.push(format!(
                        "missing header {:?} in {:?} export",
                        key, other.language
                    ));
                }
            }
        }
    }
    issues
}
