use crate::schema::{AlertRule, FiredAlert, Severity};

pub fn high_error_rate_rule() -> AlertRule {
    AlertRule::new(
        "HighErrorRate",
        Severity::Critical,
        "Run error rate exceeds 5% over the last 5 minutes",
        "https://docs.ancora.dev/runbooks/high-error-rate",
    )
}

pub fn queue_backlog_rule() -> AlertRule {
    AlertRule::new(
        "QueueBacklog",
        Severity::Warning,
        "Queue depth exceeds 100 pending runs",
        "https://docs.ancora.dev/runbooks/queue-backlog",
    )
}

pub fn worker_down_rule() -> AlertRule {
    AlertRule::new(
        "WorkerDown",
        Severity::Critical,
        "One or more workers have stopped heartbeating",
        "https://docs.ancora.dev/runbooks/worker-down",
    )
}

pub fn cost_spike_rule() -> AlertRule {
    AlertRule::new(
        "CostSpike",
        Severity::Warning,
        "Tenant cost rate exceeds 2x the 7-day average",
        "https://docs.ancora.dev/runbooks/cost-spike",
    )
}

pub fn replication_lag_rule() -> AlertRule {
    AlertRule::new(
        "ReplicationLag",
        Severity::Warning,
        "Journal replication lag exceeds RPO target",
        "https://docs.ancora.dev/runbooks/replication-lag",
    )
}

pub fn residency_violation_rule() -> AlertRule {
    AlertRule::new(
        "ResidencyViolation",
        Severity::Critical,
        "Data residency constraint violated for tenant",
        "https://docs.ancora.dev/runbooks/residency-violation",
    )
}

/// Evaluate a high-error-rate condition: fire if error_rate > threshold.
pub fn check_high_error_rate(error_rate: f64, threshold: f64, at: u64) -> Option<FiredAlert> {
    if error_rate > threshold {
        let rule = high_error_rate_rule();
        let msg = format!(
            "error_rate={:.2}% exceeds threshold={:.2}%",
            error_rate * 100.0,
            threshold * 100.0
        );
        Some(FiredAlert::from_rule(&rule, msg, at))
    } else {
        None
    }
}

pub fn check_queue_backlog(depth: u64, threshold: u64, at: u64) -> Option<FiredAlert> {
    if depth > threshold {
        let rule = queue_backlog_rule();
        let msg = format!("queue_depth={depth} exceeds threshold={threshold}");
        Some(FiredAlert::from_rule(&rule, msg, at))
    } else {
        None
    }
}

pub fn check_worker_down(heartbeat_missed: bool, worker_id: &str, at: u64) -> Option<FiredAlert> {
    if heartbeat_missed {
        let rule = worker_down_rule();
        let msg = format!("worker {worker_id} missed heartbeat");
        Some(FiredAlert::from_rule(&rule, msg, at))
    } else {
        None
    }
}

pub fn check_cost_spike(current: f64, baseline: f64, at: u64) -> Option<FiredAlert> {
    if baseline > 0.0 && current > baseline * 2.0 {
        let rule = cost_spike_rule();
        let msg = format!("cost_rate=${current:.4} exceeds 2x baseline=${baseline:.4}");
        Some(FiredAlert::from_rule(&rule, msg, at))
    } else {
        None
    }
}

pub fn check_replication_lag(lag_secs: u64, rpo_secs: u64, at: u64) -> Option<FiredAlert> {
    if lag_secs > rpo_secs {
        let rule = replication_lag_rule();
        let msg = format!("replication_lag={lag_secs}s exceeds rpo={rpo_secs}s");
        Some(FiredAlert::from_rule(&rule, msg, at))
    } else {
        None
    }
}

pub fn check_residency_violation(
    violated: bool,
    tenant_id: &str,
    region: &str,
    at: u64,
) -> Option<FiredAlert> {
    if violated {
        let rule = residency_violation_rule();
        let msg = format!("tenant {tenant_id} data written outside allowed region {region}");
        Some(FiredAlert::from_rule(&rule, msg, at))
    } else {
        None
    }
}
