#[cfg(test)]
mod tests {
    use crate::model::{ResumeDecision, RunPriority, RunState};
    use crate::store::ControlPlaneStore;

    #[test]
    fn cancel_and_resume_flow_explicit() {
        let mut store = ControlPlaneStore::new();
        let run = store.create_run("t1", RunPriority::Normal);

        // pause requires Running state
        store.runs.get_mut(&run.id).unwrap().state = RunState::Running;
        store.pause_run(&run.id).unwrap();
        assert_eq!(store.get_run(&run.id).unwrap().state, RunState::Paused);

        // approve resume re-queues the run
        store
            .resume_run(
                &run.id,
                ResumeDecision {
                    approved: true,
                    reason: None,
                },
            )
            .unwrap();
        assert_eq!(store.get_run(&run.id).unwrap().state, RunState::Queued);

        // reject resume cancels the run
        store.runs.get_mut(&run.id).unwrap().state = RunState::Paused;
        store
            .resume_run(
                &run.id,
                ResumeDecision {
                    approved: false,
                    reason: Some("rejected by operator".into()),
                },
            )
            .unwrap();
        assert_eq!(store.get_run(&run.id).unwrap().state, RunState::Cancelled);
    }
}
