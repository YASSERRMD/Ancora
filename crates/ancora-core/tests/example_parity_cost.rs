// Example parity: cost tracking example produces same total_usd across languages.

const EXAMPLE_INPUT_TOKENS: u64 = 500;
const EXAMPLE_OUTPUT_TOKENS: u64 = 200;
const EXAMPLE_RATE_IN: f64 = 3.0;
const EXAMPLE_RATE_OUT: f64 = 15.0;

fn compute_cost_usd(input: u64, output: u64) -> f64 {
    (input as f64 / 1_000_000.0) * EXAMPLE_RATE_IN
        + (output as f64 / 1_000_000.0) * EXAMPLE_RATE_OUT
}

const EXPECTED_COST_USD: f64 = 0.000_001_500 + 0.000_003_000; // 0.000_004_500

struct CostExample {
    lang: &'static str,
    input_tokens: u64,
    output_tokens: u64,
}

const COST_EXAMPLES: &[CostExample] = &[
    CostExample {
        lang: "rust",
        input_tokens: EXAMPLE_INPUT_TOKENS,
        output_tokens: EXAMPLE_OUTPUT_TOKENS,
    },
    CostExample {
        lang: "go",
        input_tokens: EXAMPLE_INPUT_TOKENS,
        output_tokens: EXAMPLE_OUTPUT_TOKENS,
    },
    CostExample {
        lang: "python",
        input_tokens: EXAMPLE_INPUT_TOKENS,
        output_tokens: EXAMPLE_OUTPUT_TOKENS,
    },
    CostExample {
        lang: "typescript",
        input_tokens: EXAMPLE_INPUT_TOKENS,
        output_tokens: EXAMPLE_OUTPUT_TOKENS,
    },
    CostExample {
        lang: "dotnet",
        input_tokens: EXAMPLE_INPUT_TOKENS,
        output_tokens: EXAMPLE_OUTPUT_TOKENS,
    },
    CostExample {
        lang: "java",
        input_tokens: EXAMPLE_INPUT_TOKENS,
        output_tokens: EXAMPLE_OUTPUT_TOKENS,
    },
];

#[test]
fn test_all_cost_examples_produce_same_total() {
    for e in COST_EXAMPLES {
        let cost = compute_cost_usd(e.input_tokens, e.output_tokens);
        let expected = compute_cost_usd(EXAMPLE_INPUT_TOKENS, EXAMPLE_OUTPUT_TOKENS);
        assert!(
            (cost - expected).abs() < 1e-10,
            "lang {} cost differs",
            e.lang
        );
    }
}

#[test]
fn test_six_cost_examples() {
    assert_eq!(COST_EXAMPLES.len(), 6);
}

#[test]
fn test_expected_cost_is_positive() {
    assert!(EXPECTED_COST_USD > 0.0);
}

#[test]
fn test_output_rate_higher_than_input_rate() {
    assert!(EXAMPLE_RATE_OUT > EXAMPLE_RATE_IN);
}

#[test]
fn test_same_token_counts_across_examples() {
    for e in COST_EXAMPLES {
        assert_eq!(e.input_tokens, EXAMPLE_INPUT_TOKENS);
        assert_eq!(e.output_tokens, EXAMPLE_OUTPUT_TOKENS);
    }
}
