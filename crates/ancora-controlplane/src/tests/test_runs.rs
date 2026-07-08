#[cfg(test)]
mod tests {
    use crate::api::runs::RunsApi;
    use crate::auth::TokenAuth;
    use crate::model::{ResumeDecision, RunPriority, RunState};
    use crate::store::ControlPlaneStore;

    fn setup() -> (ControlPlaneStore, TokenAuth) {
        let store = ControlPlaneStore::new();
        let auth = TokenAuth::new(&["test-token"]);
        (store, auth)
    }

    #[test]
    fn create_and_get_a_run() {
        let (mut store, auth) = setup();
        let mut api = RunsApi::new(&mut store, &auth);
        let run = api
            .create(Some("test-token"), "tenant-1", RunPriority::Normal)
            .unwrap();
        assert_eq!(run.state, RunState::Queued);
        let got = api.get(Some("test-token"), &run.id).unwrap();
        assert_eq!(got.id, run.id);
        assert_eq!(got.tenant_id, "tenant-1");
    }

    #[test]
    fn list_runs_with_tenant_filter() {
        let (mut store, auth) = setup();
        let mut api = RunsApi::new(&mut store, &auth);
        api.create(Some("test-token"), "tenant-1", RunPriority::Normal)
            .unwrap();
        api.create(Some("test-token"), "tenant-2", RunPriority::Normal)
            .unwrap();
        let page = api
            .list(Some("test-token"), Some("tenant-1"), None, None, 10)
            .unwrap();
        assert_eq!(page.items.len(), 1);
        assert_eq!(page.items[0].tenant_id, "tenant-1");
    }

    #[test]
    fn cancel_and_resume_flow() {
        let (mut store, auth) = setup();
        {
            let mut api = RunsApi::new(&mut store, &auth);
            let run = api
                .create(Some("test-token"), "t1", RunPriority::Normal)
                .unwrap();
            api.cancel(Some("test-token"), &run.id).unwrap();
            let got = api.get(Some("test-token"), &run.id).unwrap();
            assert_eq!(got.state, RunState::Cancelled);
        }
        // Pause + resume
        let run2 = store.create_run("t1", RunPriority::Normal);
        store.runs.get_mut(&run2.id).unwrap().state = RunState::Running;
        store.pause_run(&run2.id).unwrap();
        assert_eq!(store.get_run(&run2.id).unwrap().state, RunState::Paused);
        store
            .resume_run(
                &run2.id,
                ResumeDecision {
                    approved: true,
                    reason: None,
                },
            )
            .unwrap();
        assert_eq!(store.get_run(&run2.id).unwrap().state, RunState::Queued);
    }

    #[test]
    fn priority_ordering_honored() {
        let (mut store, auth) = setup();
        let mut api = RunsApi::new(&mut store, &auth);
        api.create(Some("test-token"), "t1", RunPriority::Low)
            .unwrap();
        let high = api
            .create(Some("test-token"), "t1", RunPriority::Critical)
            .unwrap();
        api.create(Some("test-token"), "t1", RunPriority::Normal)
            .unwrap();

        let worker = store.register_worker(10);
        let claimed = store.claim_run(&worker.id).unwrap().unwrap();
        assert_eq!(claimed.priority, RunPriority::Critical);
        assert_eq!(claimed.id, high.id);
    }

    #[test]
    fn journal_tail_returns_entries() {
        let (mut store, _auth) = setup();
        let run = store.create_run("t1", RunPriority::Normal);
        store.append_journal(&run.id, "step-1".to_string());
        store.append_journal(&run.id, "step-2".to_string());
        let entries = store.tail_journal(&run.id, 0);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].payload, "step-1");
    }

    #[test]
    fn cost_per_run_and_aggregate() {
        let (mut store, _auth) = setup();
        let run = store.create_run("t1", RunPriority::Normal);
        store.record_cost(&run.id, 100, 50);
        let summary = store.cost_per_run(&run.id).unwrap();
        assert_eq!(summary.total_tokens, 100);
        assert_eq!(summary.total_usd_micro, 50);
        let agg = store.cost_aggregate("t1");
        assert_eq!(agg.total_tokens, 100);
    }

    #[test]
    fn pagination_cursor_works() {
        let (mut store, auth) = setup();
        let mut api = RunsApi::new(&mut store, &auth);
        for _ in 0..5 {
            api.create(Some("test-token"), "t1", RunPriority::Normal)
                .unwrap();
        }
        let page1 = api.list(Some("test-token"), None, None, None, 3).unwrap();
        assert_eq!(page1.items.len(), 3);
        assert!(page1.next_cursor.is_some());
        let page2 = api
            .list(
                Some("test-token"),
                None,
                None,
                page1.next_cursor.as_ref(),
                3,
            )
            .unwrap();
        assert_eq!(page1.items.len() + page2.items.len(), 5);
    }
}
