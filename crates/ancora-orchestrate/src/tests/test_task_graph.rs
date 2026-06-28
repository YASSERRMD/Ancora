use crate::task_graph::TaskGraph;

#[test]
fn no_deps_task_is_ready_immediately() {
    let mut g = TaskGraph::new();
    g.add_task("t1", vec![]);
    let ready = g.ready_tasks();
    assert!(ready.contains(&"t1"));
}

#[test]
fn dependent_task_not_ready_until_dep_complete() {
    let mut g = TaskGraph::new();
    g.add_task("t1", vec![]);
    g.add_task("t2", vec!["t1".to_string()]);
    let ready = g.ready_tasks();
    assert!(!ready.contains(&"t2"));
    g.mark_completed("t1");
    let ready = g.ready_tasks();
    assert!(ready.contains(&"t2"));
}

#[test]
fn all_complete_returns_true_when_done() {
    let mut g = TaskGraph::new();
    g.add_task("t1", vec![]);
    g.mark_completed("t1");
    assert!(g.all_complete());
}

#[test]
fn cycle_detection() {
    let mut g = TaskGraph::new();
    g.add_task("a", vec!["b".to_string()]);
    g.add_task("b", vec!["a".to_string()]);
    assert!(g.has_cycle());
}

#[test]
fn no_cycle_in_dag() {
    let mut g = TaskGraph::new();
    g.add_task("a", vec![]);
    g.add_task("b", vec!["a".to_string()]);
    assert!(!g.has_cycle());
}
