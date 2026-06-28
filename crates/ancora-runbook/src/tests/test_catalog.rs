use crate::catalog::all_playbooks;

#[test]
fn all_playbooks_non_empty() {
    let pbs = all_playbooks();
    assert_eq!(pbs.len(), 3);
}

#[test]
fn high_error_rate_has_three_steps() {
    let pb = crate::catalog::high_error_rate();
    assert_eq!(pb.step_count(), 3);
}

#[test]
fn worker_down_has_three_steps() {
    let pb = crate::catalog::worker_down();
    assert_eq!(pb.step_count(), 3);
}

#[test]
fn queue_backlog_has_two_steps() {
    let pb = crate::catalog::queue_backlog();
    assert_eq!(pb.step_count(), 2);
}
