#[cfg(test)]
mod tests {
    use crate::scheduler::{backpressure, Backpressure, Lane};
    use ancora_controlplane::model::RunPriority;

    #[test]
    fn lane_from_priority() {
        assert_eq!(Lane::from_priority(RunPriority::Critical), Lane::Critical);
        assert_eq!(Lane::from_priority(RunPriority::High), Lane::High);
        assert_eq!(Lane::from_priority(RunPriority::Normal), Lane::Normal);
        assert_eq!(Lane::from_priority(RunPriority::Low), Lane::Low);
    }

    #[test]
    fn backpressure_none_when_low_queue() {
        assert_eq!(backpressure(2, 4, 2), Backpressure::None);
    }

    #[test]
    fn backpressure_soft_when_at_capacity() {
        // queue_depth == capacity
        assert_eq!(backpressure(8, 4, 2), Backpressure::Soft);
    }

    #[test]
    fn backpressure_hard_when_double_capacity() {
        assert_eq!(backpressure(16, 4, 2), Backpressure::Hard);
    }

    #[test]
    fn backpressure_hard_when_no_workers() {
        assert_eq!(backpressure(1, 0, 0), Backpressure::Hard);
    }

    #[test]
    fn fair_scheduler_picks_underserved_tenant() {
        use ancora_controlplane::scheduler::FairScheduler;
        let mut sched = FairScheduler::new();
        sched.set_weight("t1", 1);
        sched.set_weight("t2", 2);
        sched.record_served("t1");
        sched.record_served("t1");
        // t1 has served 2 at weight 1 (ratio 2.0), t2 has served 0 at weight 2 (ratio 0.0)
        let candidates = vec![
            ("t1".to_string(), "run-a".to_string(), RunPriority::Normal),
            ("t2".to_string(), "run-b".to_string(), RunPriority::Normal),
        ];
        let picked = sched.pick(&candidates).unwrap();
        assert_eq!(picked, "run-b", "t2 is underserved and should be picked");
    }

    #[test]
    fn priority_lane_served_first() {
        use ancora_controlplane::scheduler::FairScheduler;
        let mut sched = FairScheduler::new();
        let candidates = vec![
            (
                "t1".to_string(),
                "run-normal".to_string(),
                RunPriority::Normal,
            ),
            (
                "t1".to_string(),
                "run-critical".to_string(),
                RunPriority::Critical,
            ),
        ];
        let picked = sched.pick(&candidates).unwrap();
        assert_eq!(picked, "run-critical");
    }
}
