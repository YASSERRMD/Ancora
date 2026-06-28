//! Continuous evaluation: schedule and trigger recurring eval runs.

/// How often an eval should run.
#[derive(Debug, Clone, PartialEq)]
pub enum EvalSchedule {
    /// Run once per day at the given UTC hour.
    Daily { hour_utc: u8 },
    /// Run on every deployment event.
    OnDeploy,
    /// Run every N minutes.
    EveryMinutes(u64),
}

/// A scheduled evaluation job.
#[derive(Debug, Clone)]
pub struct ScheduledEval {
    pub eval_id: String,
    pub schedule: EvalSchedule,
    pub enabled: bool,
}

impl ScheduledEval {
    pub fn new(eval_id: impl Into<String>, schedule: EvalSchedule) -> Self {
        Self {
            eval_id: eval_id.into(),
            schedule,
            enabled: true,
        }
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }
}

/// Registry of all scheduled eval jobs.
#[derive(Debug, Default)]
pub struct ContEvalRegistry {
    jobs: Vec<ScheduledEval>,
}

impl ContEvalRegistry {
    pub fn register(&mut self, job: ScheduledEval) {
        self.jobs.push(job);
    }

    pub fn enabled_jobs(&self) -> Vec<&ScheduledEval> {
        self.jobs.iter().filter(|j| j.enabled).collect()
    }

    pub fn on_deploy_jobs(&self) -> Vec<&ScheduledEval> {
        self.jobs
            .iter()
            .filter(|j| j.enabled && j.schedule == EvalSchedule::OnDeploy)
            .collect()
    }

    pub fn total_count(&self) -> usize {
        self.jobs.len()
    }
}
