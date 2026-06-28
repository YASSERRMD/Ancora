use crate::graph::DependencyGraph;
#[test]
fn add_dependency_and_check() {
    let mut graph = DependencyGraph::new();
    graph.add_dependency("app", "libssl");
    assert!(graph.has_dependency("app", "libssl"));
    assert!(!graph.has_dependency("app", "libc"));
}
#[test]
fn direct_dependencies_returns_deps() {
    let mut graph = DependencyGraph::new();
    graph.add_dependency("app", "libssl");
    graph.add_dependency("app", "libc");
    let deps = graph.direct_dependencies("app");
    assert_eq!(deps.len(), 2);
}
#[test]
fn transitive_dependencies_resolves_chain() {
    let mut graph = DependencyGraph::new();
    graph.add_dependency("app", "libssl");
    graph.add_dependency("libssl", "libc");
    let transitive = graph.transitive_dependencies("app");
    assert!(transitive.contains("libssl"));
    assert!(transitive.contains("libc"));
}
