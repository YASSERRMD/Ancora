use crate::a2a_interop::{build_message, A2ADispatcher, A2AError, A2AMessage, A2AResponse};

fn mock_external_agent(msg: &A2AMessage) -> Result<A2AResponse, A2AError> {
    Ok(A2AResponse {
        responder_id: "mock-ext".into(),
        content: format!("mock-processed: {}", msg.content),
        correlation_id: msg.correlation_id.clone(),
    })
}

#[test]
fn a2a_interop_with_mock_external_agent() {
    let mut d = A2ADispatcher::new();
    d.register("mock-ext", mock_external_agent);

    let msg = build_message("ancora-core", "mock-ext", "classify this input");
    let resp = d.dispatch(&msg).unwrap();
    assert_eq!(resp.responder_id, "mock-ext");
    assert!(resp.content.contains("classify this input"));
    assert_eq!(resp.correlation_id, msg.correlation_id);
}

#[test]
fn a2a_unknown_recipient_returns_error() {
    let d = A2ADispatcher::new();
    let msg = build_message("a", "unknown-agent", "hello");
    assert!(matches!(
        d.dispatch(&msg),
        Err(A2AError::UnknownRecipient(_))
    ));
}

#[test]
fn a2a_multiple_agents_routed_correctly() {
    let mut d = A2ADispatcher::new();
    d.register("agent-a", |msg: &A2AMessage| {
        Ok(A2AResponse {
            responder_id: "agent-a".into(),
            content: format!("A: {}", msg.content),
            correlation_id: msg.correlation_id.clone(),
        })
    });
    d.register("agent-b", |msg: &A2AMessage| {
        Ok(A2AResponse {
            responder_id: "agent-b".into(),
            content: format!("B: {}", msg.content),
            correlation_id: msg.correlation_id.clone(),
        })
    });

    let msg_a = build_message("src", "agent-a", "for-a");
    let msg_b = build_message("src", "agent-b", "for-b");
    assert!(d.dispatch(&msg_a).unwrap().content.starts_with("A:"));
    assert!(d.dispatch(&msg_b).unwrap().content.starts_with("B:"));
}

#[test]
fn a2a_registered_agents_listed() {
    let mut d = A2ADispatcher::new();
    d.register("x", mock_external_agent);
    d.register("y", mock_external_agent);
    let names = d.registered_agents();
    assert!(names.contains(&"x"));
    assert!(names.contains(&"y"));
}
