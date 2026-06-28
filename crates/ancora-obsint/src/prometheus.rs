/// Prometheus metrics exporter: text-format scrape endpoint generation.

use crate::otlp::OtlpMetricPoint;

#[derive(Debug, Clone, PartialEq)]
pub enum MetricType {
    Counter,
    Gauge,
    Histogram,
    Summary,
    Untyped,
}

impl MetricType {
    pub fn as_str(&self) -> &'static str {
        match self {
            MetricType::Counter => "counter",
            MetricType::Gauge => "gauge",
            MetricType::Histogram => "histogram",
            MetricType::Summary => "summary",
            MetricType::Untyped => "untyped",
        }
    }
}

#[derive(Debug, Clone)]
pub struct PrometheusMetric {
    pub name: String,
    pub help: String,
    pub metric_type: MetricType,
    pub samples: Vec<PrometheusSample>,
}

#[derive(Debug, Clone)]
pub struct PrometheusSample {
    pub labels: Vec<(String, String)>,
    pub value: f64,
    pub timestamp_ms: Option<i64>,
}

impl PrometheusMetric {
    pub fn new(
        name: impl Into<String>,
        help: impl Into<String>,
        metric_type: MetricType,
    ) -> Self {
        PrometheusMetric {
            name: name.into(),
            help: help.into(),
            metric_type,
            samples: Vec::new(),
        }
    }

    pub fn add_sample(&mut self, labels: Vec<(String, String)>, value: f64) {
        self.samples.push(PrometheusSample {
            labels,
            value,
            timestamp_ms: None,
        });
    }

    /// Renders this metric to Prometheus text exposition format.
    pub fn render(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("# HELP {} {}\n", self.name, self.help));
        out.push_str(&format!(
            "# TYPE {} {}\n",
            self.name,
            self.metric_type.as_str()
        ));
        for sample in &self.samples {
            let label_str = if sample.labels.is_empty() {
                String::new()
            } else {
                let parts: Vec<String> = sample
                    .labels
                    .iter()
                    .map(|(k, v)| format!("{}=\"{}\"", k, v))
                    .collect();
                format!("{{{}}}", parts.join(","))
            };
            let ts_str = sample
                .timestamp_ms
                .map(|t| format!(" {}", t))
                .unwrap_or_default();
            out.push_str(&format!("{}{} {}{}\n", self.name, label_str, sample.value, ts_str));
        }
        out
    }
}

/// Converts a collection of OTLP metric points into Prometheus metrics (one per name).
pub fn points_to_prometheus(points: &[OtlpMetricPoint]) -> Vec<PrometheusMetric> {
    use std::collections::HashMap;
    let mut map: HashMap<String, PrometheusMetric> = HashMap::new();
    for point in points {
        let entry = map.entry(point.name.clone()).or_insert_with(|| {
            PrometheusMetric::new(
                point.name.clone(),
                format!("Metric {}", point.name),
                MetricType::Gauge,
            )
        });
        entry.add_sample(point.labels.clone(), point.value);
    }
    map.into_values().collect()
}

/// Renders a complete scrape response from multiple metrics.
pub fn render_scrape(metrics: &[PrometheusMetric]) -> String {
    metrics.iter().map(|m| m.render()).collect::<Vec<_>>().join("")
}

/// Validates a Prometheus metric name (must match [a-zA-Z_:][a-zA-Z0-9_:]*).
pub fn validate_metric_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("metric name must not be empty".to_string());
    }
    let mut chars = name.chars();
    let first = chars.next().unwrap();
    if !first.is_ascii_alphabetic() && first != '_' && first != ':' {
        return Err(format!(
            "metric name '{}' must start with [a-zA-Z_:]",
            name
        ));
    }
    for c in chars {
        if !c.is_ascii_alphanumeric() && c != '_' && c != ':' {
            return Err(format!(
                "metric name '{}' contains invalid character '{}'",
                name, c
            ));
        }
    }
    Ok(())
}
