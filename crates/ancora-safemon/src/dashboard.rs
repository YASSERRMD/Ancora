/// Safety dashboard - provides JSON-serializable summary of safety status.
///
/// Aggregates incident counts, severity distributions, and recent alerts
/// for operational visibility.

use crate::incident_log::{IncidentLog, IncidentSeverity};

#[derive(Debug, Clone)]
pub struct SeverityCount {
    pub info: usize,
    pub low: usize,
    pub medium: usize,
    pub high: usize,
    pub critical: usize,
}

impl SeverityCount {
    pub fn total(&self) -> usize {
        self.info + self.low + self.medium + self.high + self.critical
    }

    pub fn to_json(&self) -> String {
        format!(
            r#"{{"info":{},"low":{},"medium":{},"high":{},"critical":{}}}"#,
            self.info, self.low, self.medium, self.high, self.critical
        )
    }
}

#[derive(Debug, Clone)]
pub struct DashboardSnapshot {
    pub total_incidents: usize,
    pub severity_counts: SeverityCount,
    pub top_categories: Vec<(String, usize)>,
    pub alerts_fired: u64,
    pub is_healthy: bool,
}

impl DashboardSnapshot {
    pub fn to_json(&self) -> String {
        let categories_json: Vec<String> = self
            .top_categories
            .iter()
            .map(|(cat, count)| format!(r#"{{"category":"{}","count":{}}}"#, cat, count))
            .collect();

        format!(
            r#"{{"total_incidents":{},"severity_counts":{},"top_categories":[{}],"alerts_fired":{},"is_healthy":{}}}"#,
            self.total_incidents,
            self.severity_counts.to_json(),
            categories_json.join(","),
            self.alerts_fired,
            self.is_healthy,
        )
    }
}

pub struct Dashboard {
    alerts_fired: u64,
}

impl Dashboard {
    pub fn new() -> Self {
        Self { alerts_fired: 0 }
    }

    pub fn set_alerts_fired(&mut self, count: u64) {
        self.alerts_fired = count;
    }

    /// Generate a snapshot from the incident log.
    pub fn snapshot(&self, log: &IncidentLog) -> DashboardSnapshot {
        let total_incidents = log.count();

        let info = log.by_severity(&IncidentSeverity::Info).len();
        let low = log.by_severity(&IncidentSeverity::Low).len();
        let medium = log.by_severity(&IncidentSeverity::Medium).len();
        let high = log.by_severity(&IncidentSeverity::High).len();
        let critical = log.by_severity(&IncidentSeverity::Critical).len();

        let severity_counts = SeverityCount {
            info,
            low,
            medium,
            high,
            critical,
        };

        // Count incidents per category.
        let mut category_map: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();
        for incident in log.all() {
            *category_map.entry(incident.category.clone()).or_insert(0) += 1;
        }
        let mut top_categories: Vec<(String, usize)> = category_map.into_iter().collect();
        top_categories.sort_by(|a, b| b.1.cmp(&a.1));
        top_categories.truncate(10);

        let is_healthy = critical == 0 && high < 5;

        DashboardSnapshot {
            total_incidents,
            severity_counts,
            top_categories,
            alerts_fired: self.alerts_fired,
            is_healthy,
        }
    }
}

impl Default for Dashboard {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::incident_log::{IncidentLog, IncidentSeverity};

    #[test]
    fn empty_log_is_healthy() {
        let log = IncidentLog::new(100);
        let dash = Dashboard::new();
        let snap = dash.snapshot(&log);
        assert!(snap.is_healthy);
        assert_eq!(snap.total_incidents, 0);
    }

    #[test]
    fn snapshot_counts_severities() {
        let mut log = IncidentLog::new(100);
        log.log(IncidentSeverity::High, "pii", "d", "e", None);
        log.log(IncidentSeverity::Medium, "toxicity", "d", "e", None);
        log.log(IncidentSeverity::Medium, "toxicity", "d", "e", None);

        let dash = Dashboard::new();
        let snap = dash.snapshot(&log);
        assert_eq!(snap.severity_counts.high, 1);
        assert_eq!(snap.severity_counts.medium, 2);
        assert_eq!(snap.total_incidents, 3);
    }

    #[test]
    fn json_output_is_valid() {
        let mut log = IncidentLog::new(100);
        log.log(IncidentSeverity::Low, "hallucination", "d", "e", None);
        let dash = Dashboard::new();
        let snap = dash.snapshot(&log);
        let json = snap.to_json();
        assert!(json.contains("total_incidents"));
        assert!(json.contains("severity_counts"));
        assert!(json.contains("is_healthy"));
    }

    #[test]
    fn critical_incident_marks_unhealthy() {
        let mut log = IncidentLog::new(100);
        log.log(IncidentSeverity::Critical, "policy", "d", "e", None);
        let dash = Dashboard::new();
        let snap = dash.snapshot(&log);
        assert!(!snap.is_healthy);
    }
}
