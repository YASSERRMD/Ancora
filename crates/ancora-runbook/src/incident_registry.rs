use std::collections::HashMap;
use crate::incident::{Incident, Severity};

pub struct IncidentRegistry {
    incidents: HashMap<String, Incident>,
}

impl IncidentRegistry {
    pub fn new() -> Self {
        Self { incidents: HashMap::new() }
    }

    pub fn open(&mut self, incident: Incident) {
        self.incidents.insert(incident.id.clone(), incident);
    }

    pub fn get(&self, id: &str) -> Option<&Incident> {
        self.incidents.get(id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut Incident> {
        self.incidents.get_mut(id)
    }

    pub fn open_count(&self) -> usize {
        self.incidents.values().filter(|i| !i.is_resolved()).count()
    }

    pub fn by_severity(&self, severity: &Severity) -> Vec<&Incident> {
        self.incidents.values().filter(|i| &i.severity == severity).collect()
    }
}

impl Default for IncidentRegistry {
    fn default() -> Self {
        Self::new()
    }
}
