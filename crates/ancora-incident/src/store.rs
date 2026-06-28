use std::collections::HashMap;
use crate::incident::{Incident, IncidentStatus, Severity};

pub struct IncidentStore {
    incidents: HashMap<String, Incident>,
}

impl IncidentStore {
    pub fn new() -> Self { Self { incidents: HashMap::new() } }

    pub fn insert(&mut self, incident: Incident) {
        self.incidents.insert(incident.id.clone(), incident);
    }

    pub fn get(&self, id: &str) -> Option<&Incident> { self.incidents.get(id) }
    pub fn get_mut(&mut self, id: &str) -> Option<&mut Incident> { self.incidents.get_mut(id) }

    pub fn remove(&mut self, id: &str) -> Option<Incident> { self.incidents.remove(id) }

    pub fn for_tenant<'a>(&'a self, tenant_id: &str) -> Vec<&'a Incident> {
        self.incidents.values().filter(|i| i.tenant_id == tenant_id).collect()
    }

    pub fn active<'a>(&'a self) -> Vec<&'a Incident> {
        self.incidents.values().filter(|i| i.is_active()).collect()
    }

    pub fn by_severity<'a>(&'a self, severity: &Severity) -> Vec<&'a Incident> {
        self.incidents.values().filter(|i| &i.severity == severity).collect()
    }

    pub fn by_status<'a>(&'a self, status: &IncidentStatus) -> Vec<&'a Incident> {
        self.incidents.values().filter(|i| &i.status == status).collect()
    }

    pub fn count(&self) -> usize { self.incidents.len() }
}
