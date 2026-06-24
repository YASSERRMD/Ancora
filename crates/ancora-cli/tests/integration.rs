use ancora_cli_lib::spec::{GraphSpec, NodeSpec};
use std::io::Write;
use tempfile::NamedTempFile;

/// Write a graph YAML to a temp file and return the path.
fn write_graph(yaml: &str) -> NamedTempFile {
    let mut f = NamedTempFile::new().unwrap();
    f.write_all(yaml.as_bytes()).unwrap();
    f
}

#[test]
fn run_graph_minimal_spec_succeeds() {
    let yaml = "name: test-graph\nnodes: []\n";
    let f = write_graph(yaml);
    ancora_cli_lib::spec::run_graph(f.path().to_str().unwrap(), "memory").unwrap();
}

#[test]
fn run_graph_with_agent_node_succeeds() {
    let yaml = r#"
name: agent-graph
nodes:
  - id: node-1
    kind: agent
    model: mock
"#;
    let f = write_graph(yaml);
    ancora_cli_lib::spec::run_graph(f.path().to_str().unwrap(), "memory").unwrap();
}

#[test]
fn run_graph_missing_file_returns_error() {
    let result = ancora_cli_lib::spec::run_graph("/nonexistent/path.yaml", "memory");
    assert!(result.is_err());
}

#[test]
fn open_store_memory_returns_ok() {
    ancora_cli_lib::store::open_store("memory").unwrap();
}

#[test]
fn open_store_sqlite_returns_ok() {
    ancora_cli_lib::store::open_store("sqlite").unwrap();
}

#[test]
fn open_store_unknown_returns_error() {
    assert!(ancora_cli_lib::store::open_store("redis").is_err());
}

#[test]
fn graph_spec_validate_accepts_valid_graph() {
    let spec = GraphSpec {
        name: "test".into(),
        nodes: vec![
            NodeSpec { id: "a".into(), kind: "agent".into(), model: None, depends_on: vec![] },
            NodeSpec { id: "b".into(), kind: "agent".into(), model: None, depends_on: vec!["a".into()] },
        ],
    };
    spec.validate().unwrap();
}

#[test]
fn graph_spec_validate_detects_duplicate_id() {
    let spec = GraphSpec {
        name: "test".into(),
        nodes: vec![
            NodeSpec { id: "a".into(), kind: "agent".into(), model: None, depends_on: vec![] },
            NodeSpec { id: "a".into(), kind: "agent".into(), model: None, depends_on: vec![] },
        ],
    };
    assert!(spec.validate().is_err());
}

#[test]
fn graph_spec_validate_detects_unknown_dependency() {
    let spec = GraphSpec {
        name: "test".into(),
        nodes: vec![
            NodeSpec { id: "a".into(), kind: "agent".into(), model: None, depends_on: vec!["nonexistent".into()] },
        ],
    };
    assert!(spec.validate().is_err());
}

#[test]
fn graph_spec_empty_nodes_is_valid() {
    let spec = GraphSpec { name: "empty".into(), nodes: vec![] };
    spec.validate().unwrap();
}

#[test]
fn node_spec_is_agent_returns_true_for_agent_kind() {
    let n = NodeSpec { id: "n".into(), kind: "agent".into(), model: None, depends_on: vec![] };
    assert!(n.is_agent());
    assert!(!n.is_tool());
}

#[test]
fn node_spec_effective_model_defaults_to_default() {
    let n = NodeSpec { id: "n".into(), kind: "agent".into(), model: None, depends_on: vec![] };
    assert_eq!(n.effective_model(), "default");
}

#[test]
fn node_spec_effective_model_uses_explicit_model() {
    let n = NodeSpec { id: "n".into(), kind: "agent".into(), model: Some("gpt-4o".into()), depends_on: vec![] };
    assert_eq!(n.effective_model(), "gpt-4o");
}

#[test]
fn graph_spec_from_yaml_parses_inline_yaml() {
    let yaml = "name: inline-test\nnodes:\n  - id: n1\n    kind: agent\n";
    let spec = GraphSpec::from_yaml(yaml).unwrap();
    assert_eq!(spec.name, "inline-test");
    assert_eq!(spec.nodes.len(), 1);
    assert_eq!(spec.nodes[0].id, "n1");
}

#[test]
fn graph_spec_from_yaml_invalid_yaml_returns_error() {
    let bad = "name: [unclosed";
    assert!(GraphSpec::from_yaml(bad).is_err());
}

#[test]
fn graph_spec_agent_nodes_filters_correctly() {
    let spec = GraphSpec {
        name: "test".into(),
        nodes: vec![
            NodeSpec { id: "a".into(), kind: "agent".into(), model: None, depends_on: vec![] },
            NodeSpec { id: "t".into(), kind: "tool".into(), model: None, depends_on: vec![] },
        ],
    };
    assert_eq!(spec.agent_nodes().len(), 1);
    assert_eq!(spec.node_count(), 2);
}
