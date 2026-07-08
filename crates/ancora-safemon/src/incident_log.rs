/// Safety incident logging and audit trail.
///
/// Records all detected safety issues with timestamps, severity, and context
/// for audit and compliance purposes.
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum IncidentSeverity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

impl IncidentSeverity {
    pub fn as_str(&self) -> &'static str {
        match self {
            IncidentSeverity::Info => "INFO",
            IncidentSeverity::Low => "LOW",
            IncidentSeverity::Medium => "MEDIUM",
            IncidentSeverity::High => "HIGH",
            IncidentSeverity::Critical => "CRITICAL",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Incident {
    pub id: u64,
    pub severity: IncidentSeverity,
    pub category: String,
    pub description: String,
    /// Unix timestamp in seconds (or a monotonic counter in tests).
    pub timestamp: u64,
    /// Redacted excerpt of the offending text.
    pub excerpt: String,
    /// Optional agent or session ID.
    pub agent_id: Option<String>,
}

impl Incident {
    pub fn to_json(&self) -> String {
        let agent_id_json = match &self.agent_id {
            Some(id) => format!("\"{}\"", id),
            None => "null".to_string(),
        };
        format!(
            r#"{{"id":{},"severity":"{}","category":"{}","description":"{}","timestamp":{},"excerpt":"{}","agent_id":{}}}"#,
            self.id,
            self.severity.as_str(),
            self.category,
            self.description,
            self.timestamp,
            self.excerpt,
            agent_id_json,
        )
    }
}

pub struct IncidentLog {
    incidents: VecDeque<Incident>,
    next_id: u64,
    max_entries: usize,
    /// Monotonic clock substitute: incremented each log call.
    clock: u64,
}

impl IncidentLog {
    pub fn new(max_entries: usize) -> Self {
        Self {
            incidents: VecDeque::new(),
            next_id: 1,
            max_entries,
            clock: 0,
        }
    }

    /// Log a new incident and return its assigned ID.
    pub fn log(
        &mut self,
        severity: IncidentSeverity,
        category: impl Into<String>,
        description: impl Into<String>,
        excerpt: impl Into<String>,
        agent_id: Option<String>,
    ) -> u64 {
        self.clock += 1;
        let id = self.next_id;
        self.next_id += 1;

        let incident = Incident {
            id,
            severity,
            category: category.into(),
            description: description.into(),
            timestamp: self.clock,
            excerpt: excerpt.into(),
            agent_id,
        };

        if self.incidents.len() >= self.max_entries {
            self.incidents.pop_front();
        }
        self.incidents.push_back(incident);
        id
    }

    pub fn all(&self) -> impl Iterator<Item = &Incident> {
        self.incidents.iter()
    }

    pub fn count(&self) -> usize {
        self.incidents.len()
    }

    pub fn by_severity(&self, severity: &IncidentSeverity) -> Vec<&Incident> {
        self.incidents
            .iter()
            .filter(|i| &i.severity == severity)
            .collect()
    }

    pub fn find_by_id(&self, id: u64) -> Option<&Incident> {
        self.incidents.iter().find(|i| i.id == id)
    }

    /// Export all incidents as a JSON array.
    pub fn to_json(&self) -> String {
        let entries: Vec<String> = self.incidents.iter().map(|i| i.to_json()).collect();
        format!("[{}]", entries.join(","))
    }

    /// Clear all stored incidents.
    pub fn clear(&mut self) {
        self.incidents.clear();
    }
}

impl Default for IncidentLog {
    fn default() -> Self {
        Self::new(10_000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log_and_retrieve() {
        let mut log = IncidentLog::new(100);
        let id = log.log(
            IncidentSeverity::High,
            "pii",
            "Email address detected",
            "user@example.com",
            Some("agent-1".to_string()),
        );
        assert_eq!(id, 1);
        assert_eq!(log.count(), 1);
        let incident = log.find_by_id(1).unwrap();
        assert_eq!(incident.severity, IncidentSeverity::High);
    }

    #[test]
    fn max_entries_respected() {
        let mut log = IncidentLog::new(3);
        for i in 0..5 {
            log.log(
                IncidentSeverity::Info,
                "test",
                format!("desc {}", i),
                "",
                None,
            );
        }
        assert_eq!(log.count(), 3);
    }

    #[test]
    fn json_output_valid() {
        let mut log = IncidentLog::new(10);
        log.log(
            IncidentSeverity::Medium,
            "toxicity",
            "Toxic output",
            "bad word",
            None,
        );
        let json = log.to_json();
        assert!(json.starts_with('['));
        assert!(json.ends_with(']'));
        assert!(json.contains("MEDIUM"));
    }
}
