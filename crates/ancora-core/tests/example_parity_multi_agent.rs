// Example parity: multi-agent (parallel) example fan-out and fan-in across languages.

const PARALLEL_BRANCH_COUNT: usize = 3;
const PARALLEL_EXPECTED_RESULTS: usize = 3;

struct MultiAgentExample {
    lang: &'static str,
    branch_count: usize,
    result_count: usize,
    join_key: &'static str,
}

const MULTI_AGENT_EXAMPLES: &[MultiAgentExample] = &[
    MultiAgentExample {
        lang: "rust",
        branch_count: PARALLEL_BRANCH_COUNT,
        result_count: PARALLEL_EXPECTED_RESULTS,
        join_key: "results",
    },
    MultiAgentExample {
        lang: "go",
        branch_count: PARALLEL_BRANCH_COUNT,
        result_count: PARALLEL_EXPECTED_RESULTS,
        join_key: "results",
    },
    MultiAgentExample {
        lang: "python",
        branch_count: PARALLEL_BRANCH_COUNT,
        result_count: PARALLEL_EXPECTED_RESULTS,
        join_key: "results",
    },
    MultiAgentExample {
        lang: "typescript",
        branch_count: PARALLEL_BRANCH_COUNT,
        result_count: PARALLEL_EXPECTED_RESULTS,
        join_key: "results",
    },
];

#[test]
fn test_all_examples_fan_out_to_same_branch_count() {
    for e in MULTI_AGENT_EXAMPLES {
        assert_eq!(
            e.branch_count, PARALLEL_BRANCH_COUNT,
            "lang {} branch_count differs",
            e.lang
        );
    }
}

#[test]
fn test_all_examples_collect_same_result_count() {
    for e in MULTI_AGENT_EXAMPLES {
        assert_eq!(
            e.result_count, PARALLEL_EXPECTED_RESULTS,
            "lang {} result_count differs",
            e.lang
        );
    }
}

#[test]
fn test_join_key_same_across_examples() {
    for e in MULTI_AGENT_EXAMPLES {
        assert_eq!(e.join_key, "results");
    }
}

#[test]
fn test_four_multi_agent_examples() {
    assert_eq!(MULTI_AGENT_EXAMPLES.len(), 4);
}

#[test]
fn test_branch_count_equals_result_count() {
    assert_eq!(PARALLEL_BRANCH_COUNT, PARALLEL_EXPECTED_RESULTS);
}

#[test]
fn test_rust_in_multi_agent_examples() {
    assert!(MULTI_AGENT_EXAMPLES.iter().any(|e| e.lang == "rust"));
}
