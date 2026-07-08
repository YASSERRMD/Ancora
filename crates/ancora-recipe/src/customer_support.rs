use crate::format::{Recipe, RecipeStep, StepAction};
use crate::params::ParamSet;

/// Build a customer-support recipe.
pub fn build(params: &ParamSet) -> Recipe {
    let escalation_threshold: u8 = params
        .get("escalation_threshold")
        .and_then(|v| v.parse().ok())
        .unwrap_or(3);
    let product = params.get("product").unwrap_or("product");

    let mut r = Recipe::new(
        "customer-support",
        "Customer Support",
        format!(
            "Classify, respond to, and optionally escalate support tickets for '{}' (escalate after {} unresolved turns).",
            product, escalation_threshold
        ),
    );

    r.add_step(RecipeStep::new(
        "classify",
        StepAction::Classify,
        "Classify ticket by category and urgency",
    ));
    r.add_step(RecipeStep::new(
        "retrieve-kb",
        StepAction::Retrieve,
        "Retrieve relevant knowledge-base articles",
    ));
    r.add_step(RecipeStep::new(
        "respond",
        StepAction::Generate,
        "Generate a helpful, empathetic response",
    ));
    r.add_step(RecipeStep::new(
        "escalate-check",
        StepAction::Review,
        format!(
            "Escalate to human agent if turn count >= {}",
            escalation_threshold
        ),
    ));
    r
}

/// Support ticket category.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TicketCategory {
    Billing,
    Technical,
    General,
    Other(String),
}

/// Urgency level of a support ticket.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Urgency {
    Low,
    Medium,
    High,
    Critical,
}

/// A support ticket.
#[derive(Debug, Clone)]
pub struct Ticket {
    pub id: String,
    pub category: TicketCategory,
    pub urgency: Urgency,
    pub turn_count: u8,
    pub resolved: bool,
}

impl Ticket {
    pub fn new(id: impl Into<String>, category: TicketCategory, urgency: Urgency) -> Self {
        Self {
            id: id.into(),
            category,
            urgency,
            turn_count: 0,
            resolved: false,
        }
    }

    /// Determine whether the ticket should be escalated.
    pub fn should_escalate(&self, threshold: u8) -> bool {
        !self.resolved && self.turn_count >= threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::params::ParamSet;

    #[test]
    fn build_recipe_has_four_steps() {
        let params = ParamSet::default();
        let r = build(&params);
        assert_eq!(r.step_count(), 4);
        assert!(r.validate().is_ok());
    }

    #[test]
    fn escalation_logic() {
        let mut ticket = Ticket::new("T001", TicketCategory::Technical, Urgency::High);
        assert!(!ticket.should_escalate(3));
        ticket.turn_count = 3;
        assert!(ticket.should_escalate(3));
    }
}
