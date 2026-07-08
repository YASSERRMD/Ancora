pub mod dedup;
pub mod routing;
pub mod rules;
pub mod schema;
pub mod silence;

#[cfg(test)]
mod tests;

pub use dedup::AlertDedup;
pub use routing::WebhookRouter;
pub use rules::{
    check_cost_spike, check_high_error_rate, check_queue_backlog, check_replication_lag,
    check_residency_violation, check_worker_down, cost_spike_rule, high_error_rate_rule,
    queue_backlog_rule, replication_lag_rule, residency_violation_rule, worker_down_rule,
};
pub use schema::{AlertRule, FiredAlert, Severity};
pub use silence::{MaintenanceWindow, SilenceRegistry};

/// All built-in rules. Useful for registering the full catalog at startup.
pub fn all_rules() -> Vec<schema::AlertRule> {
    vec![
        high_error_rate_rule(),
        queue_backlog_rule(),
        worker_down_rule(),
        cost_spike_rule(),
        replication_lag_rule(),
        residency_violation_rule(),
    ]
}
