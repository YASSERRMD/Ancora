/// Scheduler for continuous evaluation runs.
///
/// Manages when and how often evaluation jobs are triggered,
/// supporting cron-like interval specifications and manual triggers.
use std::time::{Duration, SystemTime};

/// Interval at which evaluation jobs are triggered.
#[derive(Debug, Clone, PartialEq)]
pub enum EvalInterval {
    /// Run every N seconds (useful for testing).
    Seconds(u64),
    /// Run every N minutes.
    Minutes(u64),
    /// Run every N hours.
    Hours(u64),
}

impl EvalInterval {
    /// Convert to a `Duration`.
    pub fn to_duration(&self) -> Duration {
        match self {
            EvalInterval::Seconds(s) => Duration::from_secs(*s),
            EvalInterval::Minutes(m) => Duration::from_secs(m * 60),
            EvalInterval::Hours(h) => Duration::from_secs(h * 3600),
        }
    }
}

/// Status of a scheduled evaluation job.
#[derive(Debug, Clone, PartialEq)]
pub enum JobStatus {
    Pending,
    Running,
    Completed,
    Failed(String),
}

/// A scheduled evaluation job.
#[derive(Debug, Clone)]
pub struct EvalJob {
    pub id: String,
    pub interval: EvalInterval,
    pub last_run: Option<SystemTime>,
    pub status: JobStatus,
}

impl EvalJob {
    /// Create a new job with the given id and interval.
    pub fn new(id: impl Into<String>, interval: EvalInterval) -> Self {
        EvalJob {
            id: id.into(),
            interval,
            last_run: None,
            status: JobStatus::Pending,
        }
    }

    /// Returns true if the job is due to run based on the current time.
    pub fn is_due(&self, now: SystemTime) -> bool {
        match self.last_run {
            None => true,
            Some(last) => {
                let elapsed = now.duration_since(last).unwrap_or(Duration::ZERO);
                elapsed >= self.interval.to_duration()
            }
        }
    }

    /// Mark the job as started at the given time.
    pub fn mark_started(&mut self, at: SystemTime) {
        self.last_run = Some(at);
        self.status = JobStatus::Running;
    }

    /// Mark the job as completed.
    pub fn mark_completed(&mut self) {
        self.status = JobStatus::Completed;
    }

    /// Mark the job as failed with a reason.
    pub fn mark_failed(&mut self, reason: impl Into<String>) {
        self.status = JobStatus::Failed(reason.into());
    }
}

/// Scheduler that manages multiple evaluation jobs.
#[derive(Debug, Default)]
pub struct EvalScheduler {
    jobs: Vec<EvalJob>,
}

impl EvalScheduler {
    /// Create a new empty scheduler.
    pub fn new() -> Self {
        EvalScheduler { jobs: Vec::new() }
    }

    /// Register a job.
    pub fn register(&mut self, job: EvalJob) {
        self.jobs.push(job);
    }

    /// Return the ids of jobs that are due at the given time.
    pub fn due_jobs(&self, now: SystemTime) -> Vec<&str> {
        self.jobs
            .iter()
            .filter(|j| j.is_due(now))
            .map(|j| j.id.as_str())
            .collect()
    }

    /// Mark a job as started by id. Returns an error if not found.
    pub fn start_job(&mut self, id: &str, at: SystemTime) -> Result<(), String> {
        self.jobs
            .iter_mut()
            .find(|j| j.id == id)
            .ok_or_else(|| format!("job '{}' not found", id))
            .map(|j| j.mark_started(at))
    }

    /// Complete a job by id. Returns an error if not found.
    pub fn complete_job(&mut self, id: &str) -> Result<(), String> {
        self.jobs
            .iter_mut()
            .find(|j| j.id == id)
            .ok_or_else(|| format!("job '{}' not found", id))
            .map(|j| j.mark_completed())
    }

    /// Fail a job by id with a reason.
    pub fn fail_job(&mut self, id: &str, reason: &str) -> Result<(), String> {
        self.jobs
            .iter_mut()
            .find(|j| j.id == id)
            .ok_or_else(|| format!("job '{}' not found", id))
            .map(|j| j.mark_failed(reason))
    }

    /// Return the number of registered jobs.
    pub fn job_count(&self) -> usize {
        self.jobs.len()
    }
}
