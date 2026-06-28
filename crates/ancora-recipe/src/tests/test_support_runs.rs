use crate::customer_support::{build, Ticket, TicketCategory, Urgency};
use crate::params::ParamSet;

#[test]
fn support_recipe_builds() {
    let mut ps = ParamSet::new();
    ps.set("product", "Ancora");
    ps.set("escalation_threshold", "5");
    let r = build(&ps);
    assert!(r.validate().is_ok());
    assert_eq!(r.id, "customer-support");
}

#[test]
fn support_recipe_has_four_steps() {
    let ps = ParamSet::default();
    let r = build(&ps);
    assert_eq!(r.step_count(), 4);
}

#[test]
fn ticket_not_escalated_below_threshold() {
    let ticket = Ticket::new("T1", TicketCategory::Billing, Urgency::Medium);
    assert!(!ticket.should_escalate(3));
}

#[test]
fn resolved_ticket_never_escalates() {
    let mut ticket = Ticket::new("T2", TicketCategory::General, Urgency::Critical);
    ticket.turn_count = 10;
    ticket.resolved = true;
    assert!(!ticket.should_escalate(1));
}
