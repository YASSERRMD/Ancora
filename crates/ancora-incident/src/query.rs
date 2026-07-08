use crate::incident::{Incident, IncidentStatus, Severity};

pub struct IncidentQuery {
    pub severity: Option<Severity>,
    pub status: Option<IncidentStatus>,
    pub assignee: Option<String>,
    pub active_only: bool,
}

impl Default for IncidentQuery {
    fn default() -> Self {
        Self::new()
    }
}

impl IncidentQuery {
    pub fn new() -> Self {
        Self {
            severity: None,
            status: None,
            assignee: None,
            active_only: false,
        }
    }
    pub fn severity(mut self, s: Severity) -> Self {
        self.severity = Some(s);
        self
    }
    pub fn status(mut self, s: IncidentStatus) -> Self {
        self.status = Some(s);
        self
    }
    pub fn assignee(mut self, a: impl Into<String>) -> Self {
        self.assignee = Some(a.into());
        self
    }
    pub fn active_only(mut self) -> Self {
        self.active_only = true;
        self
    }

    pub fn run<'a>(&self, incidents: impl Iterator<Item = &'a Incident>) -> Vec<&'a Incident> {
        incidents
            .filter(|i| {
                if let Some(ref sev) = self.severity {
                    if &i.severity != sev {
                        return false;
                    }
                }
                if let Some(ref st) = self.status {
                    if &i.status != st {
                        return false;
                    }
                }
                if let Some(ref a) = self.assignee {
                    match &i.assignee {
                        Some(ia) if ia == a => {}
                        _ => return false,
                    }
                }
                if self.active_only && !i.is_active() {
                    return false;
                }
                true
            })
            .collect()
    }
}
