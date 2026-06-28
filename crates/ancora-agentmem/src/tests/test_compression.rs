use crate::compression::{ConversationCompressor, ConversationTurn};

fn turn(role: &str, tokens: u32) -> ConversationTurn {
    ConversationTurn::new(role, "content", tokens)
}

#[test]
fn no_compression_when_within_budget() {
    let c = ConversationCompressor::new(50);
    let turns = vec![turn("user", 100), turn("assistant", 100)];
    let out = c.compress(turns.clone(), 300, 2);
    assert_eq!(out.len(), 2);
}

#[test]
fn compresses_when_over_budget() {
    let c = ConversationCompressor::new(50);
    let mut turns = vec![];
    for i in 0..10 {
        turns.push(turn(if i % 2 == 0 { "user" } else { "assistant" }, 200));
    }
    let out = c.compress(turns, 500, 2);
    assert!(out.len() < 10);
    assert_eq!(out[0].role, "system"); // summary placeholder
}

#[test]
fn total_tokens_sums_correctly() {
    let turns = vec![turn("user", 100), turn("assistant", 150)];
    assert_eq!(ConversationCompressor::total_tokens(&turns), 250);
}
