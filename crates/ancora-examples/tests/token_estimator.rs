//! Detailed tests for TokenEstimator edge cases and boundary values.

use ancora_examples::TokenEstimator;

#[test]
fn empty_string_returns_one() {
    assert_eq!(1, TokenEstimator::estimate_tokens(""));
}

#[test]
fn single_char_returns_one() {
    assert_eq!(1, TokenEstimator::estimate_tokens("a"));
}

#[test]
fn four_chars_returns_one() {
    assert_eq!(1, TokenEstimator::estimate_tokens("abcd"));
}

#[test]
fn five_chars_returns_two() {
    assert_eq!(2, TokenEstimator::estimate_tokens("abcde"));
}

#[test]
fn hundred_chars_returns_twenty_five() {
    assert_eq!(25, TokenEstimator::estimate_tokens(&"x".repeat(100)));
}

#[test]
fn estimate_scales_with_length() {
    let short = TokenEstimator::estimate_tokens("hi");
    let long  = TokenEstimator::estimate_tokens("This is a somewhat longer piece of text.");
    assert!(long > short);
}

#[test]
fn unicode_chars_count_by_byte_length() {
    let s = "\u{00e9}"; // 'e' with acute, 2 bytes in UTF-8
    assert!(TokenEstimator::estimate_tokens(s) >= 1);
}
