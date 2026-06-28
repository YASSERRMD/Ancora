#[cfg(test)]
mod tests {
    use crate::bounds::ScaleBounds;
    use crate::cooldown::Cooldown;
    use crate::policy::ScalePolicy;
    use crate::simulator::Simulator;

    fn make_policy() -> ScalePolicy {
        let mut p = ScalePolicy::new(ScaleBounds::new(1, 10));
        p.cooldown = Cooldown::new(0, 0);
        p
    }

    #[test]
    fn simulator_scales_up_on_load() {
        let mut sim = Simulator::new(make_policy(), 2);
        sim.tick(20, 2, 2);
        assert!(sim.current_workers() > 2, "should have scaled up");
    }

    #[test]
    fn simulator_scales_down_on_idle() {
        let mut sim = Simulator::new(make_policy(), 6);
        for _ in 0..3 {
            sim.tick(0, 0, 4);
        }
        assert!(sim.current_workers() < 6, "should have scaled down");
    }

    #[test]
    fn simulator_respects_bounds() {
        let mut sim = Simulator::new(make_policy(), 1);
        // Drive with zero load - should not go below min=1
        for _ in 0..5 {
            sim.tick(0, 0, 4);
        }
        assert!(sim.current_workers() >= 1);
    }

    #[test]
    fn simulator_log_records_decisions() {
        let mut sim = Simulator::new(make_policy(), 2);
        sim.tick(10, 2, 2);
        assert!(!sim.log().is_empty());
    }

    #[test]
    fn simulator_validates_load_profile() {
        // Simulate a ramp: 0->10->5->0 queue depth over ticks
        let profile = vec![(0, 0), (10, 4), (10, 8), (5, 4), (0, 0)];
        let mut sim = Simulator::new(make_policy(), 2);
        for (queue, active) in profile {
            sim.tick(queue, active, 4);
        }
        let log = sim.log();
        assert!(log.len() == 5);
        // At least one scale-up should have occurred
        let scaled = log.iter().any(|s| s.decision.is_scale_up());
        assert!(scaled, "load profile should trigger at least one scale-up");
    }
}
