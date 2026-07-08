use crate::bounds::ScaleBounds;
use crate::decision::ScaleDecision;
use crate::metrics::AutoscaleMetrics;
use crate::policy::ScalePolicy;
use crate::signals::ScaleSignal;

/// Simulates a load profile against a scaling policy.
pub struct Simulator {
    policy: ScalePolicy,
    current_workers: usize,
    log: Vec<ScaleSignal>,
}

impl Simulator {
    pub fn new(policy: ScalePolicy, initial_workers: usize) -> Self {
        Simulator {
            policy,
            current_workers: initial_workers,
            log: vec![],
        }
    }

    /// Feed one metrics snapshot and record the decision.
    pub fn tick(
        &mut self,
        queue_depth: usize,
        active_runs: usize,
        concurrency: usize,
    ) -> &ScaleDecision {
        let utilization =
            AutoscaleMetrics::compute_utilization(active_runs, self.current_workers, concurrency);
        let m = AutoscaleMetrics {
            queue_depth,
            worker_count: self.current_workers,
            active_runs,
            concurrency_per_worker: concurrency,
            last_run_latency_ms: 0,
            utilization,
        };
        let decision = self.policy.evaluate(&m);
        match &decision {
            ScaleDecision::ScaleUp { by } => self.current_workers += by,
            ScaleDecision::ScaleDown { by } => {
                self.current_workers = self.current_workers.saturating_sub(*by)
            }
            ScaleDecision::NoOp { .. } => {}
        }
        self.log.push(ScaleSignal::from_decision(
            decision,
            self.current_workers,
            queue_depth,
            utilization,
        ));
        &self.log.last().unwrap().decision
    }

    pub fn current_workers(&self) -> usize {
        self.current_workers
    }

    pub fn log(&self) -> &[ScaleSignal] {
        &self.log
    }
}
