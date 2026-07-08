/// Extended context assembly tests -- all offline.

#[cfg(test)]
mod context_ext_tests {
    use crate::embedders::context::*;

    #[test]
    fn estimate_tokens_with_zero_factor_returns_zero() {
        // words * 0 = 0 (ceiling of 0 is 0).
        assert_eq!(estimate_tokens("hello world", 0.0), 0);
    }

    #[test]
    fn estimate_tokens_with_large_factor() {
        // 4 words * 10.0 = 40 tokens.
        let t = estimate_tokens("a b c d", 10.0);
        assert_eq!(t, 40);
    }

    #[test]
    fn estimate_tokens_default_single_word() {
        // 1 word * 1.5 = 1.5, ceil = 2.
        let t = estimate_tokens_default("hello");
        assert_eq!(t, 2);
    }

    #[test]
    fn assembler_empty_passages_produces_empty_text() {
        let a = ContextAssembler::new(9999);
        let ctx = a.assemble(&[]);
        assert_eq!(ctx.text, "");
        assert_eq!(ctx.passages_included, 0);
        assert_eq!(ctx.passages_dropped, 0);
    }

    #[test]
    fn assembler_header_contains_passage_number() {
        let a = ContextAssembler::new(100_000);
        let ctx = a.assemble(&["hello"]);
        assert!(ctx.text.contains("[Passage 1]"), "text: {}", ctx.text);
    }

    #[test]
    fn assembler_header_numbers_increment() {
        let a = ContextAssembler::new(100_000);
        let ctx = a.assemble(&["a", "b", "c"]);
        assert!(ctx.text.contains("[Passage 1]"), "text: {}", ctx.text);
        assert!(ctx.text.contains("[Passage 2]"), "text: {}", ctx.text);
        assert!(ctx.text.contains("[Passage 3]"), "text: {}", ctx.text);
    }

    #[test]
    fn assembler_estimated_tokens_is_consistent() {
        let a = ContextAssembler::new(100_000).without_headers();
        let ctx = a.assemble(&["one two three"]);
        let expected = estimate_tokens("one two three", 1.5);
        assert_eq!(ctx.estimated_tokens, expected);
    }

    #[test]
    fn assembler_separator_in_multi_passage_output() {
        let a = ContextAssembler::new(100_000).with_separator("|||");
        let ctx = a.assemble(&["first", "second"]);
        assert!(ctx.text.contains("|||"), "text: {}", ctx.text);
    }

    #[test]
    fn assembler_skips_passages_exceeding_budget_not_just_first() {
        // Budget is tight; only first passage fits. Second is dropped. Third, if any, also.
        let a = ContextAssembler::new(4).without_headers();
        let ctx = a.assemble(&["word", "another word", "yet another"]);
        assert!(ctx.passages_included >= 1);
        let total = ctx.passages_included + ctx.passages_dropped;
        assert_eq!(total, 3);
    }

    #[test]
    fn assembler_tokens_per_word_override() {
        let a1 = ContextAssembler::new(100_000).with_tokens_per_word(1.0);
        let a2 = ContextAssembler::new(100_000).with_tokens_per_word(2.0);
        let ctx1 = a1.assemble(&["hello world"]);
        let ctx2 = a2.assemble(&["hello world"]);
        assert!(
            ctx2.estimated_tokens > ctx1.estimated_tokens,
            "higher factor should cost more"
        );
    }

    #[test]
    fn assembled_context_passages_included_plus_dropped_equals_total() {
        let a = ContextAssembler::new(10);
        let ctx = a.assemble(&["one two three", "four five six", "seven eight nine"]);
        let total = ctx.passages_included + ctx.passages_dropped;
        assert_eq!(total, 3, "included + dropped should equal 3");
    }
}
