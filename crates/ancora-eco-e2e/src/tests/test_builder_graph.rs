use crate::builder_e2e::{Node, PluginGraph};

#[test]
fn test_graph_builder_produces_running_graph() {
    let mut graph = PluginGraph::new();
    graph
        .add_node(Node::new(1, "Ingest", "ingest-plugin"))
        .unwrap();
    graph
        .add_node(Node::new(2, "Process", "process-plugin"))
        .unwrap();
    graph
        .add_node(Node::new(3, "Output", "output-plugin"))
        .unwrap();
    graph.add_edge(1, 2).unwrap();
    graph.add_edge(2, 3).unwrap();
    assert_eq!(graph.node_count(), 3);
    assert_eq!(graph.edge_count(), 2);
    let order = graph.topological_order().expect("must produce valid order");
    assert_eq!(order, vec![1, 2, 3]);
}

#[test]
fn test_duplicate_node_rejected() {
    let mut graph = PluginGraph::new();
    graph.add_node(Node::new(1, "A", "plugin-a")).unwrap();
    let result = graph.add_node(Node::new(1, "B", "plugin-b"));
    assert!(result.is_err());
}

#[test]
fn test_cycle_detection() {
    let mut graph = PluginGraph::new();
    graph.add_node(Node::new(1, "A", "p")).unwrap();
    graph.add_node(Node::new(2, "B", "q")).unwrap();
    graph.add_edge(1, 2).unwrap();
    graph.add_edge(2, 1).unwrap();
    assert!(graph.has_cycle());
}

#[test]
fn test_missing_node_edge_fails() {
    let mut graph = PluginGraph::new();
    graph.add_node(Node::new(1, "A", "plugin")).unwrap();
    let result = graph.add_edge(1, 99);
    assert!(result.is_err());
}
