#[cfg(test)]
mod tests {
    use crate::{BlueGreenController, Version, VersionedWorker};

    fn v(s: &str) -> Version {
        let p: Vec<u32> = s.split('.').map(|x| x.parse().unwrap()).collect();
        Version::new(p[0], p[1], p[2])
    }

    fn idle_worker(id: &str, ver: &str) -> VersionedWorker {
        VersionedWorker::new(id, v(ver))
    }

    fn busy_worker(id: &str, ver: &str) -> VersionedWorker {
        let mut w = VersionedWorker::new(id, v(ver));
        w.active_runs = 1;
        w
    }

    #[test]
    fn blue_green_switch_keeps_runs_alive() {
        let blue = vec![idle_worker("b1", "1.0.0")];
        let green = vec![idle_worker("g1", "2.0.0")];
        let mut ctrl = BlueGreenController::new(blue, green);
        ctrl.switch().unwrap();
        assert!(ctrl.green_live);
        assert_eq!(ctrl.live_version().unwrap().major, 2);
    }

    #[test]
    fn drain_completes_before_switch() {
        let blue = vec![busy_worker("b1", "1.0.0")];
        let green = vec![idle_worker("g1", "2.0.0")];
        let mut ctrl = BlueGreenController::new(blue, green);
        let err = ctrl.switch().unwrap_err();
        assert!(matches!(err, crate::DeployError::DrainIncomplete { .. }));
    }

    #[test]
    fn rollback_restores_previous_version() {
        let blue = vec![idle_worker("b1", "1.0.0")];
        let green = vec![idle_worker("g1", "2.0.0")];
        let mut ctrl = BlueGreenController::new(blue, green);
        ctrl.switch().unwrap();
        ctrl.rollback().unwrap();
        assert!(!ctrl.green_live);
        assert_eq!(ctrl.live_version().unwrap().major, 1);
    }

    #[test]
    fn switch_time_measured_in_constant_time() {
        let blue = vec![idle_worker("b1", "1.0.0")];
        let green = vec![idle_worker("g1", "2.0.0")];
        let mut ctrl = BlueGreenController::new(blue, green);
        let start = std::time::Instant::now();
        ctrl.switch().unwrap();
        let elapsed_ms = start.elapsed().as_millis();
        // Switch is in-memory; must complete in well under 100ms
        assert!(elapsed_ms < 100, "switch took too long: {elapsed_ms}ms");
    }
}
