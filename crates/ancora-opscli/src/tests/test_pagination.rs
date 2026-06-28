use crate::run_store::{RunEntry, RunStatus};
use crate::pagination::paginate;

fn make_entries(n: usize) -> Vec<RunEntry> {
    (0..n)
        .map(|i| RunEntry {
            run_id: format!("run-{i}"),
            tenant_id: "t".into(),
            status: RunStatus::Pending,
            worker_id: None,
            created_at_secs: i as u64,
        })
        .collect()
}

#[test]
fn first_page() {
    let entries = make_entries(10);
    let page = paginate(&entries, 0, 3);
    assert_eq!(page.items.len(), 3);
    assert_eq!(page.total, 10);
    assert!(page.has_next());
}

#[test]
fn last_page_no_next() {
    let entries = make_entries(5);
    let page = paginate(&entries, 1, 3);
    assert_eq!(page.items.len(), 2);
    assert!(!page.has_next());
}

#[test]
fn empty_store() {
    let entries: Vec<RunEntry> = vec![];
    let page = paginate(&entries, 0, 10);
    assert_eq!(page.total, 0);
    assert!(!page.has_next());
}
