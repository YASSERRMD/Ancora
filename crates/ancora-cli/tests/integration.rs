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
