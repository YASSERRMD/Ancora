use crate::assignment::assign;
use crate::experiment::{Experiment, Metric, MetricKind, Variant};

/// Verify that replaying the same sequence of assignments produces identical results.
#[test]
fn replay_produces_identical_assignments() {
    let experiment = Experiment::new(
        "replay-exp",
        "Replay test",
        vec![
            Variant::new("a", 0.33),
            Variant::new("b", 0.34),
            Variant::new("c", 0.33),
        ],
        Metric::new("quality", MetricKind::Maximize),
    )
    .unwrap();

    let keys: Vec<String> = (0..200).map(|i| format!("subject-{i}")).collect();

    let run1: Vec<String> = keys
        .iter()
        .map(|k| assign(&experiment, k).variant_name)
        .collect();
    let run2: Vec<String> = keys
        .iter()
        .map(|k| assign(&experiment, k).variant_name)
        .collect();

    assert_eq!(
        run1, run2,
        "second replay produced different assignments than the first"
    );
}

#[test]
fn assignment_does_not_depend_on_order() {
    let experiment = Experiment::new(
        "order-exp",
        "Order independence",
        vec![Variant::new("x", 0.5), Variant::new("y", 0.5)],
        Metric::new("m", MetricKind::Minimize),
    )
    .unwrap();

    let keys: Vec<String> = (0..50).map(|i| format!("u{i}")).collect();
    let forward: Vec<_> = keys
        .iter()
        .map(|k| assign(&experiment, k).variant_name.clone())
        .collect();

    let mut backward_keys = keys.clone();
    backward_keys.reverse();
    let backward: Vec<_> = backward_keys
        .iter()
        .map(|k| assign(&experiment, k).variant_name.clone())
        .collect();

    // Re-sort backward to original order so we can compare element-wise.
    let mut backward_map: std::collections::HashMap<&str, &str> = std::collections::HashMap::new();
    for (k, v) in backward_keys.iter().zip(backward.iter()) {
        backward_map.insert(k.as_str(), v.as_str());
    }
    for (key, expected) in keys.iter().zip(forward.iter()) {
        assert_eq!(
            backward_map[key.as_str()],
            expected.as_str(),
            "assignment for {key} changed based on order"
        );
    }
}
