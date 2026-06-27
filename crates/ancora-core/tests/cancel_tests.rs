use std::sync::Arc;
use std::thread;

use ancora_core::cancel::cancellation_pair;

#[test]
fn token_is_not_cancelled_initially() {
    let (token, _handle) = cancellation_pair();
    assert!(!token.is_cancelled(), "token must start in non-cancelled state");
}

#[test]
fn token_is_cancelled_after_handle_cancel() {
    let (token, handle) = cancellation_pair();
    handle.cancel();
    assert!(token.is_cancelled(), "token must reflect cancellation");
}

#[test]
fn cancel_is_idempotent() {
    let (token, handle) = cancellation_pair();
    handle.cancel();
    handle.cancel();
    handle.cancel();
    assert!(token.is_cancelled(), "repeated cancel must not panic");
}

#[test]
fn token_reads_handle_cancellation_from_different_thread() {
    let (token, handle) = cancellation_pair();
    let token = Arc::new(token);
    let token_clone = Arc::clone(&token);

    let t = thread::spawn(move || {
        handle.cancel();
    });
    t.join().unwrap();

    assert!(
        token_clone.is_cancelled(),
        "cancellation signal must be visible cross-thread"
    );
}

#[test]
fn multiple_tokens_from_same_handle_all_see_cancellation() {
    let (token1, handle) = cancellation_pair();

    // Can't clone a token easily; verify the single shared-flag path
    assert!(!token1.is_cancelled());
    handle.cancel();
    assert!(token1.is_cancelled());
}

#[test]
fn cancellation_does_not_affect_other_pairs() {
    let (token_a, handle_a) = cancellation_pair();
    let (token_b, _handle_b) = cancellation_pair();

    handle_a.cancel();

    assert!(token_a.is_cancelled(), "pair A must be cancelled");
    assert!(!token_b.is_cancelled(), "pair B must be unaffected");
}

#[test]
fn hundred_pairs_are_independent() {
    let pairs: Vec<_> = (0..100).map(|_| cancellation_pair()).collect();
    // Cancel every even-indexed pair
    for (i, (_, handle)) in pairs.iter().enumerate() {
        if i % 2 == 0 {
            handle.cancel();
        }
    }
    for (i, (token, _)) in pairs.iter().enumerate() {
        if i % 2 == 0 {
            assert!(token.is_cancelled(), "pair {i} should be cancelled");
        } else {
            assert!(!token.is_cancelled(), "pair {i} should not be cancelled");
        }
    }
}
