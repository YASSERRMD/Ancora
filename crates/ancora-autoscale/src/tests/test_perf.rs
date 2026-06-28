#[cfg(test)]
mod tests {
    use crate::bounds::ScaleBounds;
    use crate::cooldown::Cooldown;
    use crate::perf::measure_decision_latency;
    use crate::policy::ScalePolicy;

    #[test]
    fn decision_latency_under_threshold() {
        let mut p = ScalePolicy::new(ScaleBounds::new(1, 20));
        p.cooldown = Cooldown::new(0, 0);
        let avg_us = measure_decision_latency(&mut p, 1000);
        // A pure-Rust evaluation should complete well under 1 ms
        assert!(avg_us < 1000, "decision took {}us avg, expected < 1000us", avg_us);
    }
}
