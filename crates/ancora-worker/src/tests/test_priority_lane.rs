#[cfg(test)]
mod tests {
    use ancora_controlplane::model::RunPriority;
    use ancora_controlplane::store::ControlPlaneStore;

    #[test]
    fn priority_lane_served_first_in_store() {
        let mut store = ControlPlaneStore::new();
        store.create_run("t1", RunPriority::Low);
        let high_run = store.create_run("t1", RunPriority::Critical);
        store.create_run("t1", RunPriority::Normal);

        let w = store.register_worker(10);
        let claimed = store.claim_run(&w.id).unwrap().unwrap();
        assert_eq!(claimed.id, high_run.id, "Critical run must be served first");
        assert_eq!(claimed.priority, RunPriority::Critical);
    }

    #[test]
    fn high_before_normal_before_low() {
        let mut store = ControlPlaneStore::new();
        store.create_run("t1", RunPriority::Low);
        let high = store.create_run("t1", RunPriority::High);
        store.create_run("t1", RunPriority::Normal);

        let w = store.register_worker(10);
        let claimed = store.claim_run(&w.id).unwrap().unwrap();
        assert_eq!(claimed.id, high.id);
        assert_eq!(claimed.priority, RunPriority::High);
    }
}
