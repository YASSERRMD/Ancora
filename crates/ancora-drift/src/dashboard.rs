//! Drift dashboard JSON generation.
//!
//! Produces a JSON-serialisable snapshot of current drift state for consumption
//! by monitoring dashboards (Grafana, custom UIs, etc.) without any external
//! serialisation library dependency.

use crate::alerting::Alert;
use std::fmt::Write;

/// A named numeric metric.
#[derive(Debug, Clone)]
pub struct Metric {
    pub name: String,
    pub value: f64,
    pub unit: String,
}

/// A complete dashboard snapshot.
#[derive(Debug, Clone)]
pub struct DashboardSnapshot {
    /// ISO-8601 timestamp (supplied by the caller).
    pub timestamp: String,
    pub metrics: Vec<Metric>,
    pub alerts: Vec<Alert>,
    /// Overall health label.
    pub health: HealthStatus,
}

/// Coarse overall health derived from alert severities.
#[derive(Debug, Clone, PartialEq)]
pub enum HealthStatus {
    Ok,
    Degraded,
    Incident,
}

impl HealthStatus {
    fn as_str(&self) -> &'static str {
        match self {
            HealthStatus::Ok => "ok",
            HealthStatus::Degraded => "degraded",
            HealthStatus::Incident => "incident",
        }
    }
}

impl DashboardSnapshot {
    pub fn new(timestamp: impl Into<String>, metrics: Vec<Metric>, alerts: Vec<Alert>) -> Self {
        use crate::alerting::Severity;
        let health = if alerts.iter().any(|a| a.severity == Severity::Critical) {
            HealthStatus::Incident
        } else if alerts.iter().any(|a| a.severity == Severity::Warning) {
            HealthStatus::Degraded
        } else {
            HealthStatus::Ok
        };
        Self {
            timestamp: timestamp.into(),
            metrics,
            alerts,
            health,
        }
    }

    /// Render to a JSON string without external dependencies.
    pub fn to_json(&self) -> String {
        let mut s = String::new();
        s.push_str("{\n");
        let _ = writeln!(s, "  \"timestamp\": \"{}\",", escape_json(&self.timestamp));
        let _ = writeln!(s, "  \"health\": \"{}\",", self.health.as_str());

        // metrics array
        s.push_str("  \"metrics\": [\n");
        for (i, m) in self.metrics.iter().enumerate() {
            let comma = if i + 1 < self.metrics.len() { "," } else { "" };
            let _ = writeln!(
                s,
                "    {{\"name\": \"{}\", \"value\": {}, \"unit\": \"{}\"}}{}",
                escape_json(&m.name),
                m.value,
                escape_json(&m.unit),
                comma
            );
        }
        s.push_str("  ],\n");

        // alerts array
        s.push_str("  \"alerts\": [\n");
        for (i, a) in self.alerts.iter().enumerate() {
            let comma = if i + 1 < self.alerts.len() { "," } else { "" };
            let metric_field = match a.metric_value {
                Some(v) => format!(", \"metric_value\": {v}"),
                None => String::new(),
            };
            let _ = writeln!(
                s,
                "    {{\"severity\": \"{}\", \"kind\": \"{}\", \"message\": \"{}\"{}}}{}",
                a.severity,
                escape_json(&a.kind),
                escape_json(&a.message),
                metric_field,
                comma
            );
        }
        s.push_str("  ]\n");
        s.push('}');
        s
    }
}

/// Minimal JSON string escaping (backslash, double-quote, control chars).
fn escape_json(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c => out.push(c),
        }
    }
    out
}

/// Parse a very simple subset of JSON to validate structure (used in tests).
/// Returns the string value for a top-level key in a flat JSON object.
pub fn extract_top_level_str<'a>(json: &'a str, key: &str) -> Option<&'a str> {
    let search = format!("\"{}\":", key);
    let start = json.find(search.as_str())?;
    let after_colon = json[start + search.len()..].trim_start();
    if let Some(inner) = after_colon.strip_prefix('"') {
        let end = inner.find('"')?;
        Some(&inner[..end])
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::alerting::{Alert, Severity};

    #[test]
    fn empty_snapshot_serialises() {
        let snap = DashboardSnapshot::new("2026-01-01T00:00:00Z", vec![], vec![]);
        let json = snap.to_json();
        assert!(json.contains("\"health\""));
        assert!(json.contains("\"ok\""));
    }

    #[test]
    fn snapshot_health_reflects_alerts() {
        let alerts = vec![Alert::new(Severity::Critical, "cost_drift", "spike")];
        let snap = DashboardSnapshot::new("ts", vec![], alerts);
        assert_eq!(snap.health, HealthStatus::Incident);
        let json = snap.to_json();
        assert!(json.contains("\"incident\""));
    }

    #[test]
    fn extract_top_level_str_works() {
        let snap = DashboardSnapshot::new("2026-06-28T12:00:00Z", vec![], vec![]);
        let json = snap.to_json();
        let ts = extract_top_level_str(&json, "timestamp");
        assert_eq!(ts, Some("2026-06-28T12:00:00Z"));
    }
}
