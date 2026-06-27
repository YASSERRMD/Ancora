use ancora_core::graph::{Edge, Graph, Node, NodeKind, NodeSpec};
use ancora_proto::ancora::AgentSpec as ProtoAgentSpec;

fn agent_node(id: &str) -> Node {
    Node {
        id: id.to_string(),
        kind: NodeKind::Agent,
        model_id: None,
        spec: NodeSpec::Agent(ProtoAgentSpec {
            name: id.to_string(),
            model_id: "mock".to_string(),
            instructions: String::new(),
            output_schema_json: String::new(),
            tools: vec![],
            max_steps: 1,
            model_retry: None,
            model_params_json: String::new(),
        }),
    }
}

fn edge(from: &str, to: &str) -> Edge {
    Edge { from: from.to_string(), to: to.to_string(), condition: None }
}

fn single_node_graph() -> Graph {
    Graph {
        id: "g".to_string(),
        nodes: vec![agent_node("a")],
        edges: vec![],
        entry_node: "a".to_string(),
    }
}

#[test]
fn single_node_graph_is_valid() {
    single_node_graph().validate().unwrap();
}

#[test]
fn two_node_linear_graph_is_valid() {
    Graph {
        id: "g".to_string(),
        nodes: vec![agent_node("a"), agent_node("b")],
        edges: vec![edge("a", "b")],
        entry_node: "a".to_string(),
    }
    .validate()
    .unwrap();
}

#[test]
fn empty_node_list_is_rejected() {
    let err = Graph {
        id: "g".to_string(),
        nodes: vec![],
        edges: vec![],
        entry_node: "x".to_string(),
    }
    .validate()
    .unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("no nodes"), "error must mention empty nodes");
}

#[test]
fn duplicate_node_id_is_rejected() {
    let err = Graph {
        id: "g".to_string(),
        nodes: vec![agent_node("a"), agent_node("a")],
        edges: vec![],
        entry_node: "a".to_string(),
    }
    .validate()
    .unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("duplicate"), "error must mention duplicate");
    assert!(msg.contains("'a'"), "error must name the duplicate id");
}

#[test]
fn missing_entry_node_is_rejected() {
    let err = Graph {
        id: "g".to_string(),
        nodes: vec![agent_node("a")],
        edges: vec![],
        entry_node: "missing".to_string(),
    }
    .validate()
    .unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("missing"), "error must mention missing entry node");
}

#[test]
fn edge_with_unknown_source_is_rejected() {
    let err = Graph {
        id: "g".to_string(),
        nodes: vec![agent_node("a"), agent_node("b")],
        edges: vec![edge("ghost", "b")],
        entry_node: "a".to_string(),
    }
    .validate()
    .unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("ghost") || msg.contains("source"), "error must name the bad source");
}

#[test]
fn edge_with_unknown_target_is_rejected() {
    let err = Graph {
        id: "g".to_string(),
        nodes: vec![agent_node("a"), agent_node("b")],
        edges: vec![edge("a", "phantom")],
        entry_node: "a".to_string(),
    }
    .validate()
    .unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("phantom") || msg.contains("target"), "error must name the bad target");
}

#[test]
fn self_loop_is_accepted_by_validate() {
    Graph {
        id: "g".to_string(),
        nodes: vec![agent_node("loop-node")],
        edges: vec![edge("loop-node", "loop-node")],
        entry_node: "loop-node".to_string(),
    }
    .validate()
    .unwrap();
}

#[test]
fn fan_out_graph_is_valid() {
    Graph {
        id: "g".to_string(),
        nodes: vec![agent_node("root"), agent_node("a"), agent_node("b"), agent_node("merge")],
        edges: vec![edge("root", "a"), edge("root", "b"), edge("a", "merge"), edge("b", "merge")],
        entry_node: "root".to_string(),
    }
    .validate()
    .unwrap();
}

#[test]
fn large_chain_is_valid() {
    let n = 100usize;
    let nodes: Vec<Node> = (0..n).map(|i| agent_node(&format!("n{}", i))).collect();
    let edges: Vec<Edge> = (0..n - 1).map(|i| edge(&format!("n{}", i), &format!("n{}", i + 1))).collect();
    Graph {
        id: "g".to_string(),
        nodes,
        edges,
        entry_node: "n0".to_string(),
    }
    .validate()
    .unwrap();
}
