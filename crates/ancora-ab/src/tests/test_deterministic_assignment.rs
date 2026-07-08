use crate::assignment::{assign, deterministic_bucket};
use crate::experiment::{Experiment, Metric, MetricKind, Variant};

#[test]
fn same_key_always_gets_same_variant() {
    let experiment = Experiment::new(
        "det-test",
        "Determinism check",
        vec![Variant::new("control", 0.5), Variant::new("treatment", 0.5)],
        Metric::new("score", MetricKind::Maximize),
    )
    .unwrap();

    let key = "stable-user-999";
    let first = assign(&experiment, key).variant_name;
    for _ in 0..20 {
        let again = assign(&experiment, key).variant_name;
        assert_eq!(first, again, "assignment changed for the same key");
    }
}

#[test]
fn bucket_is_stable_across_calls() {
    let b1 = deterministic_bucket("user-abc", "exp-001");
    let b2 = deterministic_bucket("user-abc", "exp-001");
    let b3 = deterministic_bucket("user-abc", "exp-001");
    assert_eq!(b1, b2);
    assert_eq!(b2, b3);
}

#[test]
fn different_experiments_can_produce_different_variants() {
    let mk_exp = |id: &str| {
        Experiment::new(
            id,
            "variant check",
            vec![Variant::new("a", 0.5), Variant::new("b", 0.5)],
            Metric::new("m", MetricKind::Maximize),
        )
        .unwrap()
    };

    let exp1 = mk_exp("exp-one");
    let exp2 = mk_exp("exp-two");
    // It is very likely that some users get different variants across experiments.
    let mut differ = false;
    for i in 0..100 {
        let key = format!("u-{i}");
        if assign(&exp1, &key).variant_name != assign(&exp2, &key).variant_name {
            differ = true;
            break;
        }
    }
    assert!(
        differ,
        "expected at least one user to differ across experiments"
    );
}
