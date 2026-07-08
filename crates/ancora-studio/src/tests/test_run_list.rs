use crate::run_list::{RunList, RunListFilter, RunStatus, RunSummary};

fn make_run(id: &str, status: RunStatus, label: &str) -> RunSummary {
    RunSummary {
        id: id.into(),
        label: label.into(),
        status,
        started_at: 1000,
        duration_ms: Some(200),
        total_cost_usd: Some(0.001),
        step_count: 2,
        tags: vec!["test".into()],
    }
}

#[test]
fn test_run_list_renders_all() {
    let list = RunList::new(vec![
        make_run("r1", RunStatus::Completed, "first"),
        make_run("r2", RunStatus::Failed, "second"),
    ]);
    assert_eq!(list.all().len(), 2);
}

#[test]
fn test_run_list_search_by_label() {
    let list = RunList::new(vec![
        make_run("r1", RunStatus::Completed, "alpha"),
        make_run("r2", RunStatus::Completed, "beta"),
    ]);
    let f = RunListFilter {
        query: "beta".into(),
        ..Default::default()
    };
    let results = list.filter(&f);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "r2");
}

#[test]
fn test_run_list_empty_filter_returns_all() {
    let list = RunList::new(vec![
        make_run("r1", RunStatus::Completed, "alpha"),
        make_run("r2", RunStatus::Running, "beta"),
    ]);
    let f = RunListFilter::default();
    assert_eq!(list.filter(&f).len(), 2);
}

#[test]
fn test_run_list_get_by_id() {
    let list = RunList::new(vec![make_run("r42", RunStatus::Completed, "x")]);
    assert!(list.get("r42").is_some());
    assert!(list.get("r99").is_none());
}
