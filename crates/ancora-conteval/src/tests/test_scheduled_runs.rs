use crate::scheduler::{EvalInterval, EvalJob, EvalScheduler, JobStatus};
use std::time::{Duration, SystemTime};

#[test]
fn test_new_job_is_due_immediately() {
    let job = EvalJob::new("job-1", EvalInterval::Minutes(5));
    let now = SystemTime::now();
    assert!(job.is_due(now));
}

#[test]
fn test_job_not_due_immediately_after_run() {
    let mut job = EvalJob::new("job-1", EvalInterval::Hours(1));
    let now = SystemTime::now();
    job.mark_started(now);
    // Should not be due again right away.
    assert!(!job.is_due(now));
}

#[test]
fn test_job_due_after_interval_elapsed() {
    let mut job = EvalJob::new("job-1", EvalInterval::Seconds(10));
    let t0 = SystemTime::UNIX_EPOCH + Duration::from_secs(1_000);
    job.mark_started(t0);
    let t1 = t0 + Duration::from_secs(11);
    assert!(job.is_due(t1));
}

#[test]
fn test_scheduler_registers_and_finds_due_jobs() {
    let mut sched = EvalScheduler::new();
    sched.register(EvalJob::new("j1", EvalInterval::Minutes(1)));
    sched.register(EvalJob::new("j2", EvalInterval::Hours(24)));
    let now = SystemTime::now();
    let due = sched.due_jobs(now);
    assert_eq!(due.len(), 2); // both new jobs are due
}

#[test]
fn test_scheduler_complete_job() {
    let mut sched = EvalScheduler::new();
    sched.register(EvalJob::new("j1", EvalInterval::Seconds(1)));
    let now = SystemTime::now();
    sched.start_job("j1", now).unwrap();
    sched.complete_job("j1").unwrap();
    // status is complete - verify via job_count still 1
    assert_eq!(sched.job_count(), 1);
}

#[test]
fn test_scheduler_fail_job() {
    let mut sched = EvalScheduler::new();
    sched.register(EvalJob::new("j1", EvalInterval::Seconds(1)));
    let now = SystemTime::now();
    sched.start_job("j1", now).unwrap();
    sched.fail_job("j1", "network error").unwrap();
    assert_eq!(sched.job_count(), 1);
}

#[test]
fn test_scheduler_missing_job_returns_error() {
    let mut sched = EvalScheduler::new();
    let result = sched.start_job("nonexistent", SystemTime::now());
    assert!(result.is_err());
}

#[test]
fn test_interval_durations() {
    assert_eq!(
        EvalInterval::Seconds(30).to_duration(),
        Duration::from_secs(30)
    );
    assert_eq!(
        EvalInterval::Minutes(5).to_duration(),
        Duration::from_secs(300)
    );
    assert_eq!(
        EvalInterval::Hours(2).to_duration(),
        Duration::from_secs(7200)
    );
}

#[test]
fn test_job_status_transitions() {
    let mut job = EvalJob::new("j", EvalInterval::Seconds(1));
    assert_eq!(job.status, JobStatus::Pending);
    job.mark_started(SystemTime::now());
    assert_eq!(job.status, JobStatus::Running);
    job.mark_completed();
    assert_eq!(job.status, JobStatus::Completed);
}
