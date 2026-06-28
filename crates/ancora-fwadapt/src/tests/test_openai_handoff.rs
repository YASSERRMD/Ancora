use crate::openai_agents::{
    build_handoff, HandoffBridge, HandoffError, OpenAIAgentResult,
};

fn translator_agent(ctx: &str) -> Result<OpenAIAgentResult, HandoffError> {
    Ok(OpenAIAgentResult {
        agent_id: "translator".into(),
        output: format!("[translated] {}", ctx),
        finished: true,
    })
}

#[test]
fn openai_sdk_handoff_works() {
    let mut bridge = HandoffBridge::new();
    bridge.register_agent("translator", translator_agent);

    let handoff = build_handoff("translator", "requires translation", "Bonjour le monde");
    let result = bridge.execute_handoff(&handoff).unwrap();
    assert!(result.finished);
    assert!(result.output.contains("Bonjour le monde"));
    assert_eq!(result.agent_id, "translator");
}

#[test]
fn openai_handoff_no_target_returns_error() {
    let bridge = HandoffBridge::new();
    let h = build_handoff("nonexistent", "reason", "ctx");
    assert!(matches!(
        bridge.execute_handoff(&h),
        Err(HandoffError::NoTargetRegistered(_))
    ));
}

#[test]
fn openai_bridge_lists_agents() {
    let mut bridge = HandoffBridge::new();
    bridge.register_agent("agent1", translator_agent);
    bridge.register_agent("agent2", translator_agent);
    let names = bridge.agent_names();
    assert!(names.contains(&"agent1"));
    assert!(names.contains(&"agent2"));
}

#[test]
fn openai_handoff_fields_preserved() {
    let h = build_handoff("tgt", "my reason", "my context");
    assert_eq!(h.target_agent, "tgt");
    assert_eq!(h.reason, "my reason");
    assert_eq!(h.context, "my context");
}
