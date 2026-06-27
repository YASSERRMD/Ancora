use ancora_core::error::AncoraError;
use ancora_core::replay::detect_divergence;

#[test]
fn identical_sequences_pass() {
    let keys = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    detect_divergence(&keys, &keys).unwrap();
}

#[test]
fn empty_sequences_pass() {
    detect_divergence(&[], &[]).unwrap();
}

#[test]
fn observed_subset_passes_divergence_check() {
    let expected = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let observed = vec!["a".to_string(), "b".to_string()];
    // observed is shorter: this is valid (run paused mid-way)
    detect_divergence(&expected, &observed).unwrap();
}

#[test]
fn observed_longer_than_expected_triggers_divergence() {
    let expected = vec!["a".to_string()];
    let observed = vec!["a".to_string(), "extra".to_string()];
    let err = detect_divergence(&expected, &observed).unwrap_err();
    assert!(
        matches!(err, AncoraError::Nondeterminism { seq: 1, .. }),
        "must flag divergence at seq 1"
    );
}

#[test]
fn first_key_mismatch_triggers_divergence_at_seq_zero() {
    let expected = vec!["step-1".to_string()];
    let observed = vec!["step-X".to_string()];
    let err = detect_divergence(&expected, &observed).unwrap_err();
    assert!(
        matches!(err, AncoraError::Nondeterminism { seq: 0, .. }),
        "first mismatch must be at seq 0"
    );
}

#[test]
fn middle_key_mismatch_reports_correct_seq() {
    let expected = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let observed = vec!["a".to_string(), "WRONG".to_string(), "c".to_string()];
    let err = detect_divergence(&expected, &observed).unwrap_err();
    assert!(
        matches!(err, AncoraError::Nondeterminism { seq: 1, .. }),
        "mismatch at position 1 must yield seq 1"
    );
}

#[test]
fn divergence_error_includes_expected_and_got_values() {
    let expected = vec!["expected-key".to_string()];
    let observed = vec!["got-key".to_string()];
    let err = detect_divergence(&expected, &observed).unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("expected-key"), "error must mention expected key");
    assert!(msg.contains("got-key"), "error must mention observed key");
}

#[test]
fn long_matching_prefix_followed_by_divergence_pinpoints_correct_position() {
    let n = 100;
    let expected: Vec<String> = (0..n).map(|i| format!("step-{}", i)).collect();
    let mut observed = expected.clone();
    observed[n - 1] = "step-WRONG".to_string();

    let err = detect_divergence(&expected, &observed).unwrap_err();
    assert!(
        matches!(err, AncoraError::Nondeterminism { seq, .. } if seq == (n as u64 - 1)),
        "divergence must be at the last position"
    );
}

#[test]
fn fully_empty_expected_with_nonempty_observed_triggers_divergence() {
    let observed = vec!["unexpected".to_string()];
    let err = detect_divergence(&[], &observed).unwrap_err();
    assert!(
        matches!(err, AncoraError::Nondeterminism { seq: 0, .. }),
        "any observed step beyond empty journal must trigger divergence"
    );
}
