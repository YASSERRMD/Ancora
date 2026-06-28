use crate::subagent::SubAgentNode;
use crate::skill::SkillScope;
use serde_json::json;

#[test]
fn subagent_invoked_as_node_succeeds() {
    let node = SubAgentNode::new("n1", "agent-a", json!({"query": "hello"}));
    let result = node.invoke(&SkillScope::ReadOnly).unwrap();
    assert_eq!(result.node_id, "n1");
    assert_eq!(result.output["status"], "ok");
}

#[test]
fn subagent_local_write_scope_allowed() {
    let node = SubAgentNode::new("n2", "agent-b", json!({}));
    assert!(node.invoke(&SkillScope::LocalWrite).is_ok());
}

#[test]
fn subagent_unrestricted_scope_blocked() {
    let node = SubAgentNode::new("n3", "agent-c", json!({}));
    assert!(node.invoke(&SkillScope::Unrestricted).is_err());
}
