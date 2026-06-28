use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionItem {
    pub description: String,
    pub owner: String,
    pub due_secs: u64,
    pub completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostMortem {
    pub incident_id: String,
    pub impact_summary: String,
    pub timeline: Vec<TimelineEvent>,
    pub root_cause: String,
    pub contributing_factors: Vec<String>,
    pub action_items: Vec<ActionItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEvent {
    pub at_secs: u64,
    pub description: String,
}

impl PostMortem {
    pub fn new(incident_id: &str, root_cause: &str) -> Self {
        Self {
            incident_id: incident_id.to_string(),
            impact_summary: String::new(),
            timeline: vec![],
            root_cause: root_cause.to_string(),
            contributing_factors: vec![],
            action_items: vec![],
        }
    }

    pub fn add_event(&mut self, at: u64, description: &str) {
        self.timeline.push(TimelineEvent { at_secs: at, description: description.to_string() });
    }

    pub fn add_action(&mut self, description: &str, owner: &str, due: u64) {
        self.action_items.push(ActionItem {
            description: description.to_string(),
            owner: owner.to_string(),
            due_secs: due,
            completed: false,
        });
    }

    pub fn open_actions(&self) -> usize {
        self.action_items.iter().filter(|a| !a.completed).count()
    }
}
