use crate::call_graph::CallGraph;

#[test]
fn no_deps_can_run_parallel() {
    let g = CallGraph::new();
    assert!(g.can_run_parallel("a", "b"));
}

#[test]
fn direct_dep_cannot_run_parallel() {
    let mut g = CallGraph::new();
    g.add_dependency("b", "a"); // b depends on a
    assert!(!g.can_run_parallel("a", "b"));
}

#[test]
fn no_cycle_in_dag() {
    let mut g = CallGraph::new();
    g.add_dependency("b", "a");
    g.add_dependency("c", "b");
    assert!(!g.has_cycle());
}

#[test]
fn cycle_detected() {
    let mut g = CallGraph::new();
    g.add_dependency("b", "a");
    g.add_dependency("a", "b"); // cycle
    assert!(g.has_cycle());
}
