use ancora_lh::{
    BackgroundRun, Checkpoint, CheckpointCadence, Deadline, ExternalSignal, ProgressStore,
    RunDashboard, ScheduledWakeup, SignalQueue, Throttle,
};

pub fn run_background_agent_example() {
    let mut run = BackgroundRun::new("bg-1", 0);
    run.start();

    let wakeup = ScheduledWakeup::new("bg-1", 50);
    let mut cadence = CheckpointCadence::new(10);
    let mut progress = ProgressStore::default();
    progress.init("bg-1", 5);

    let mut signals = SignalQueue::default();
    let deadline = Deadline::new("bg-1", 1000);
    let mut throttle = Throttle::new(5);

    for tick in 0u64..=60 {
        deadline.check(tick).expect("should not exceed deadline");
        throttle.try_op(tick).ok();

        if cadence.should_checkpoint(tick) {
            let mut cp = Checkpoint::new("bg-1", tick);
            cp.set("tick", &tick.to_string());
            assert!(cp.get("tick").is_some());
        }

        progress.advance("bg-1", tick);

        if tick == 30 {
            signals.inject(ExternalSignal {
                run_id: "bg-1".to_string(),
                kind: "pause".to_string(),
                payload: "{}".to_string(),
                tick,
            });
        }

        if let Some(sig) = signals.pop() {
            assert_eq!(sig.kind, "pause");
        }

        if tick == 40 {
            run.sleep_until(50);
        }

        if wakeup.should_fire(tick) {
            run.wake(tick);
        }
    }

    run.apply_effect("send-summary");
    assert!(!run.apply_effect("send-summary"));

    run.complete();

    let p = progress.get("bg-1").unwrap();
    let dashboard = RunDashboard::from(&run, Some(p));
    assert_eq!(dashboard.state_label, "completed");
    assert_eq!(dashboard.pct_complete, 100.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn background_agent_example_runs() {
        run_background_agent_example();
    }
}
