use crate::customer_support::{ResponseTemplate, SupportEngine, TicketStatus};

#[test]
fn support_engine_routes_ticket_offline() {
    let templates = vec![
        ResponseTemplate::new("billing", "For billing issues please visit the billing portal."),
        ResponseTemplate::new("refund", "Refund requests are processed within 5-7 business days."),
    ];
    let mut engine = SupportEngine::new(templates);

    let id = engine.submit("Billing question", "I was charged twice this month.");
    let response = engine.auto_respond(id).unwrap();
    assert!(response.contains("billing portal"), "should match billing template");
}

#[test]
fn unmatched_ticket_gets_default_response() {
    let mut engine = SupportEngine::new(vec![]);
    let id = engine.submit("Random question", "What is the meaning of life?");
    let response = engine.auto_respond(id).unwrap();
    assert!(response.contains("agent"), "should return default fallback");
}

#[test]
fn ticket_lifecycle_state_transitions() {
    let mut engine = SupportEngine::new(vec![]);
    let id = engine.submit("Bug report", "App crashes on launch.");
    assert_eq!(engine.open_count(), 1);
    engine.resolve_ticket(id);
    assert_eq!(engine.open_count(), 0);
}

#[test]
fn resolve_nonexistent_ticket_returns_false() {
    let mut engine = SupportEngine::new(vec![]);
    assert!(!engine.resolve_ticket(999));
}
