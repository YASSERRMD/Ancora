/// Determinism: divergence is detected when code changes alter activity output.
use ancora_core::replay::detect_divergence;

fn expected_keys() -> Vec<String> {
    vec!["compute:step1".into(), "compute:step2".into(), "compute:step3".into()]
}

#[test] fn same_keys_do_not_diverge() {
    let keys = expected_keys();
    assert!(detect_divergence(&keys, &keys).is_ok());
}

#[test] fn missing_key_triggers_divergence() {
    let expected = expected_keys();
    let observed = vec!["compute:step1".into(), "compute:step2".into()];
    assert!(detect_divergence(&expected, &observed).is_err(), "missing key must trigger divergence");
}

#[test] fn extra_key_triggers_divergence() {
    let expected = expected_keys();
    let mut observed = expected_keys();
    observed.push("compute:step4".into());
    assert!(detect_divergence(&expected, &observed).is_err(), "extra key must trigger divergence");
}

#[test] fn wrong_key_at_same_position_triggers_divergence() {
    let expected = expected_keys();
    let observed = vec!["compute:step1".into(), "compute:CHANGED".into(), "compute:step3".into()];
    assert!(detect_divergence(&expected, &observed).is_err(), "changed key must trigger divergence");
}

#[test] fn empty_expected_with_observed_triggers_divergence() {
    let empty: Vec<String> = vec![];
    let observed = vec!["compute:step1".into()];
    assert!(detect_divergence(&empty, &observed).is_err());
}

#[test] fn both_empty_does_not_diverge() {
    let empty: Vec<String> = vec![];
    assert!(detect_divergence(&empty, &empty).is_ok());
}
