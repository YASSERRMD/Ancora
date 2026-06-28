use crate::attribute::AttributeSet;
use crate::condition::Condition;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Effect {
    Allow,
    Deny,
}

#[derive(Debug, Clone)]
pub struct Policy {
    pub id: String,
    pub effect: Effect,
    pub actions: Vec<String>,
    pub condition: Condition,
    pub priority: u32,
}

impl Policy {
    pub fn new(id: impl Into<String>, effect: Effect, actions: Vec<String>, condition: Condition) -> Self {
        Self { id: id.into(), effect, actions, condition, priority: 100 }
    }

    pub fn with_priority(mut self, priority: u32) -> Self {
        self.priority = priority;
        self
    }

    pub fn applies_to_action(&self, action: &str) -> bool {
        self.actions.iter().any(|a| a == action || a == "*")
    }

    pub fn evaluate(&self, action: &str, subject: &AttributeSet, resource: &AttributeSet, env: &AttributeSet) -> Option<&Effect> {
        if !self.applies_to_action(action) { return None; }
        if self.condition.evaluate(subject, resource, env) { Some(&self.effect) } else { None }
    }
}

#[derive(Debug, Default)]
pub struct PolicyStore {
    policies: Vec<Policy>,
}

impl PolicyStore {
    pub fn new() -> Self { Self::default() }

    pub fn add(&mut self, policy: Policy) {
        self.policies.push(policy);
        self.policies.sort_by_key(|p| p.priority);
    }

    pub fn policies(&self) -> &[Policy] { &self.policies }

    pub fn count(&self) -> usize { self.policies.len() }
}
