use crate::control::{ComplianceControl, ControlStatus};
use crate::framework::{ControlId, Framework};
use std::collections::HashMap;

pub struct ControlRegistry {
    controls: HashMap<String, ComplianceControl>,
}

impl ControlRegistry {
    pub fn new() -> Self {
        Self {
            controls: HashMap::new(),
        }
    }

    pub fn register(&mut self, control: ComplianceControl) {
        self.controls.insert(control.id.0.clone(), control);
    }

    pub fn get(&self, id: &ControlId) -> Option<&ComplianceControl> {
        self.controls.get(&id.0)
    }

    pub fn get_mut(&mut self, id: &ControlId) -> Option<&mut ComplianceControl> {
        self.controls.get_mut(&id.0)
    }

    pub fn count(&self) -> usize {
        self.controls.len()
    }

    pub fn for_framework(&self, framework: &Framework) -> Vec<&ComplianceControl> {
        self.controls
            .values()
            .filter(|c| &c.framework == framework)
            .collect()
    }

    pub fn by_status(&self, status: &ControlStatus) -> Vec<&ComplianceControl> {
        self.controls
            .values()
            .filter(|c| &c.status == status)
            .collect()
    }

    pub fn compliant_count(&self) -> usize {
        self.controls.values().filter(|c| c.is_compliant()).count()
    }

    pub fn non_compliant_count(&self) -> usize {
        self.controls
            .values()
            .filter(|c| c.status == ControlStatus::NonCompliant)
            .count()
    }

    pub fn all(&self) -> impl Iterator<Item = &ComplianceControl> {
        self.controls.values()
    }
}

impl Default for ControlRegistry {
    fn default() -> Self {
        Self::new()
    }
}
