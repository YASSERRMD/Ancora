use crate::incident::{Incident, Severity};
use crate::runbook::{Runbook, RunbookStep};

pub struct IncidentBuilder {
    id: String,
    tenant_id: String,
    title: String,
    severity: Severity,
    tick: u64,
}

impl IncidentBuilder {
    pub fn new(
        id: impl Into<String>,
        tenant_id: impl Into<String>,
        title: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            tenant_id: tenant_id.into(),
            title: title.into(),
            severity: Severity::Medium,
            tick: 0,
        }
    }

    pub fn severity(mut self, s: Severity) -> Self {
        self.severity = s;
        self
    }
    pub fn tick(mut self, t: u64) -> Self {
        self.tick = t;
        self
    }

    pub fn build(self) -> Incident {
        Incident::new(
            self.id,
            self.tenant_id,
            self.title,
            self.severity,
            self.tick,
        )
    }
}

pub struct RunbookBuilder {
    id: String,
    name: String,
    incident_id: String,
    steps: Vec<RunbookStep>,
}

impl RunbookBuilder {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        incident_id: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            incident_id: incident_id.into(),
            steps: Vec::new(),
        }
    }

    pub fn step(
        mut self,
        id: impl Into<String>,
        title: impl Into<String>,
        desc: impl Into<String>,
    ) -> Self {
        self.steps.push(RunbookStep::new(id, title, desc));
        self
    }

    pub fn build(self) -> Runbook {
        let mut rb = Runbook::new(self.id, self.name, self.incident_id);
        for step in self.steps {
            rb.add_step(step);
        }
        rb
    }
}
