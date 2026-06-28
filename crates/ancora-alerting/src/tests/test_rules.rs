#[cfg(test)]
mod tests {
    use crate::rules::*;

    #[test]
    fn high_error_rate_fires_above_threshold() {
        let alert = check_high_error_rate(0.06, 0.05, 1000);
        assert!(alert.is_some());
        let a = alert.unwrap();
        assert_eq!(a.rule_name, "HighErrorRate");
        assert!(!a.runbook_url.is_empty());
    }

    #[test]
    fn high_error_rate_does_not_fire_below_threshold() {
        assert!(check_high_error_rate(0.04, 0.05, 1000).is_none());
    }

    #[test]
    fn queue_backlog_fires_above_threshold() {
        let alert = check_queue_backlog(101, 100, 1000);
        assert!(alert.is_some());
        assert_eq!(alert.unwrap().rule_name, "QueueBacklog");
    }

    #[test]
    fn worker_down_fires_on_missed_heartbeat() {
        let alert = check_worker_down(true, "w-1", 1000);
        assert!(alert.is_some());
        assert_eq!(alert.unwrap().rule_name, "WorkerDown");
    }

    #[test]
    fn worker_down_does_not_fire_when_healthy() {
        assert!(check_worker_down(false, "w-1", 1000).is_none());
    }

    #[test]
    fn cost_spike_fires_above_2x_baseline() {
        let alert = check_cost_spike(0.21, 0.10, 1000);
        assert!(alert.is_some());
        assert_eq!(alert.unwrap().rule_name, "CostSpike");
    }

    #[test]
    fn cost_spike_does_not_fire_at_1x() {
        assert!(check_cost_spike(0.10, 0.10, 1000).is_none());
    }

    #[test]
    fn replication_lag_fires_when_exceeds_rpo() {
        let alert = check_replication_lag(61, 60, 1000);
        assert!(alert.is_some());
    }

    #[test]
    fn residency_violation_fires() {
        let alert = check_residency_violation(true, "tenant-a", "eu-west-1", 1000);
        assert!(alert.is_some());
        assert_eq!(alert.unwrap().rule_name, "ResidencyViolation");
    }

    #[test]
    fn residency_violation_does_not_fire_when_clean() {
        assert!(check_residency_violation(false, "tenant-a", "eu-west-1", 1000).is_none());
    }

    #[test]
    fn every_rule_has_runbook_link() {
        let rules = vec![
            high_error_rate_rule(),
            queue_backlog_rule(),
            worker_down_rule(),
            cost_spike_rule(),
            replication_lag_rule(),
            residency_violation_rule(),
        ];
        for r in &rules {
            assert!(r.has_runbook(), "Rule {} missing runbook", r.name);
        }
    }
}
