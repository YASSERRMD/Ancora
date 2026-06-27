use std::sync::{Arc, Mutex};

use ancora_core::cancel::cancellation_pair;
use ancora_core::error::AncoraError;
use ancora_core::executor::{GraphExecutor, NodeExecutor};
use ancora_core::graph::{Edge, Graph, Node, NodeKind, NodeSpec};
use ancora_core::journal::MemoryStore;
use ancora_proto::ancora::AgentSpec as ProtoSpec;

fn agent_node(id: &str) -> Node {
    Node {
        id: id.to_string(),
        kind: NodeKind::Agent,
        model_id: None,
        spec: NodeSpec::Agent(ProtoSpec {
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

struct EchoExecutor;

impl NodeExecutor for EchoExecutor {
    fn execute(&self, node: &Node, input: &str) -> Result<String, AncoraError> {
        Ok(format!("{}->{}", node.id, input))
    }
}

struct FailExecutor;

impl NodeExecutor for FailExecutor {
    fn execute(&self, _node: &Node, _input: &str) -> Result<String, AncoraError> {
        Err(AncoraError::ModelUnreachable("mock fail".into()))
    }
}

struct StepCountingExecutor {
    count: Arc<Mutex<usize>>,
}

impl NodeExecutor for StepCountingExecutor {
    fn execute(&self, node: &Node, input: &str) -> Result<String, AncoraError> {
        let mut c = self.count.lock().unwrap();
        *c += 1;
        Ok(format!("{}-{}", node.id, *c))
    }
}

fn make_store() -> Arc<MemoryStore> {
    Arc::new(MemoryStore::new())
}

#[test]
fn single_node_graph_executes_and_returns_output() {
    let graph = Graph {
        id: "g".into(),
        nodes: vec![agent_node("only")],
        edges: vec![],
        entry_node: "only".into(),
    };
    let store = make_store();
    let mut exec = GraphExecutor::new(graph, "run-1", store);
    let output = exec.run("hello", &EchoExecutor).unwrap();
    assert_eq!(output, "only->hello");
}

#[test]
fn two_node_chain_passes_output_as_next_input() {
    let graph = Graph {
        id: "g".into(),
        nodes: vec![agent_node("a"), agent_node("b")],
        edges: vec![edge("a", "b")],
        entry_node: "a".into(),
    };
    let store = make_store();
    let mut exec = GraphExecutor::new(graph, "run-2", store);
    let output = exec.run("start", &EchoExecutor).unwrap();
    assert_eq!(output, "b->a->start");
}

#[test]
fn node_failure_propagates_as_error() {
    let graph = Graph {
        id: "g".into(),
        nodes: vec![agent_node("fail-node")],
        edges: vec![],
        entry_node: "fail-node".into(),
    };
    let store = make_store();
    let mut exec = GraphExecutor::new(graph, "run-3", store);
    let err = exec.run("x", &FailExecutor).unwrap_err();
    assert!(matches!(err, AncoraError::ModelUnreachable(_)));
}

#[test]
fn cancelled_before_first_node_returns_cancelled_error() {
    let graph = Graph {
        id: "g".into(),
        nodes: vec![agent_node("a")],
        edges: vec![],
        entry_node: "a".into(),
    };
    let store = make_store();
    let (token, handle) = cancellation_pair();
    handle.cancel(); // cancel before run starts

    let mut exec = GraphExecutor::new(graph, "run-4", store).with_cancel(token);
    let err = exec.run("input", &EchoExecutor).unwrap_err();
    assert!(matches!(err, AncoraError::Cancelled(_)), "must return Cancelled error");
}

#[test]
fn compensation_called_on_cancel() {
    let graph = Graph {
        id: "g".into(),
        nodes: vec![agent_node("a")],
        edges: vec![],
        entry_node: "a".into(),
    };
    let store = make_store();
    let (token, handle) = cancellation_pair();
    handle.cancel();

    let compensated = Arc::new(Mutex::new(false));
    let flag = Arc::clone(&compensated);

    let mut exec = GraphExecutor::new(graph, "run-5", store).with_cancel(token);
    exec.register_compensation("a", move || {
        *flag.lock().unwrap() = true;
    });
    let _ = exec.run("input", &EchoExecutor);
    assert!(*compensated.lock().unwrap(), "compensation must be called on cancel");
}

#[test]
fn cost_summary_starts_at_zero() {
    let graph = Graph {
        id: "g".into(),
        nodes: vec![agent_node("a")],
        edges: vec![],
        entry_node: "a".into(),
    };
    let store = make_store();
    let exec = GraphExecutor::new(graph, "run-6", store);
    let summary = exec.cost_summary();
    assert_eq!(summary.total_cost_usd, 0.0);
}

#[test]
fn three_node_chain_executes_all_nodes_in_order() {
    let count = Arc::new(Mutex::new(0usize));
    let executor = StepCountingExecutor { count: Arc::clone(&count) };

    let graph = Graph {
        id: "g".into(),
        nodes: vec![agent_node("n1"), agent_node("n2"), agent_node("n3")],
        edges: vec![edge("n1", "n2"), edge("n2", "n3")],
        entry_node: "n1".into(),
    };
    let store = make_store();
    let mut exec = GraphExecutor::new(graph, "run-7", store);
    exec.run("start", &executor).unwrap();
    assert_eq!(*count.lock().unwrap(), 3, "all three nodes must execute");
}
