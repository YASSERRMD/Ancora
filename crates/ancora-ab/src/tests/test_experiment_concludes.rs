use crate::analysis::welch_t_test;
use crate::experiment::{Experiment, Metric, MetricKind, Variant};
use crate::lifecycle::LifecycleManager;
use crate::outcome::{Observation, OutcomeStore};

#[test]
fn experiment_concludes_with_winner() {
    let experiment = Experiment::new(
        "conclude-test",
        "Conclude with winner",
        vec![Variant::new("control", 0.5), Variant::new("treatment", 0.5)],
        Metric::new("success_rate", MetricKind::Maximize),
    )
    .unwrap();

    let mut lifecycle = LifecycleManager::new("conclude-test");
    lifecycle.start().unwrap();
    assert!(lifecycle.is_running());

    // Populate clearly distinct data.
    let mut store = OutcomeStore::new();
    for i in 0..50 {
        store.record(Observation::new(
            "conclude-test",
            format!("c{i}"),
            "control",
            0.5,
        ));
        store.record(Observation::new(
            "conclude-test",
            format!("t{i}"),
            "treatment",
            0.9,
        ));
    }

    let ctrl = store.stats_for_variant("conclude-test", "control").unwrap();
    let trt = store
        .stats_for_variant("conclude-test", "treatment")
        .unwrap();
    let sig = welch_t_test(&ctrl, &trt, 0.05).unwrap();

    let winner = if sig.is_significant && sig.mean_difference > 0.0 {
        Some("treatment".to_string())
    } else {
        None
    };

    lifecycle.conclude(winner).unwrap();
    assert!(!lifecycle.is_running());
    assert_eq!(lifecycle.winner(), Some("treatment"));
}

#[test]
fn inconclusive_experiment_concludes_with_no_winner() {
    let mut lifecycle = LifecycleManager::new("inconclusive");
    lifecycle.start().unwrap();
    lifecycle.conclude(None).unwrap();
    assert!(lifecycle.winner().is_none());
}

#[test]
fn cannot_conclude_twice() {
    use crate::lifecycle::LifecycleError;
    let mut lifecycle = LifecycleManager::new("double-conclude");
    lifecycle.start().unwrap();
    lifecycle.conclude(Some("a".to_string())).unwrap();
    let err = lifecycle.conclude(None).unwrap_err();
    assert_eq!(err, LifecycleError::AlreadyConcluded);
}
